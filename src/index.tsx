import {
  ButtonItem,
  definePlugin,
  //DialogButton,
  //Menu,
  //MenuItem,
  PanelSection,
  PanelSectionRow,
  //Router,
  ServerAPI,
  //showContextMenu,
  staticClasses,
  SliderField,
  ToggleField,
  //NotchLabel
  gamepadDialogClasses,
  joinClassNames,
} from "decky-frontend-lib";
import { VFC, useState } from "react";
import { GiWashingMachine } from "react-icons/gi";

import { get_value, set_value } from "usdpl-front";
import * as backend from "./backend";
import {register_for_steam_events, unregister_for_steam_events} from "./steam_events";

const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

const DISPLAY_KEY = "display";
const VALUE_KEY = "value";

let items: backend.CElement[] = [];
let about: backend.CAbout | null = null;

let update = () => {};

let updateTasks: (() => void)[] = [];

let displayErrors: number[] = [];
const DISPLAY_ERROR_ABORT_THRESHHOLD = 8;

function displayCallback(index: number) {
  return (newVal: backend.CDisplayResponse) => {
    if (newVal != null) {
      switch (newVal.result) {
        case "value":
          displayErrors[index] = 0;
          let val = newVal as backend.CValueResult;
          console.log("CAYLON: Got display for " + index.toString(), val);
          backend.log(backend.CLogLevel.DEBUG, "Got display for " + index.toString());
          set_value(DISPLAY_KEY + index.toString(), val.value);
          break;
        case "error":
          displayErrors[index]++;
          let err = newVal as backend.CErrorResult;
          console.warn("CAYLON: Got display error for " + index.toString(), err);
          backend.log(backend.CLogLevel.WARN, "Got display error for " + index.toString());
          break;
        default:
          console.error("CAYLON: Got invalid display response for " + index.toString(), newVal);
          backend.log(backend.CLogLevel.ERROR, "Got invalid display response for " + index.toString());
          break;
      }
    } else {
      displayErrors[index]++;
      console.warn("CAYLON: Ignoring null display result for " + index.toString());
      backend.log(backend.CLogLevel.WARN, "Ignoring null display result for " + index.toString());
    }
    if (displayErrors[index] < DISPLAY_ERROR_ABORT_THRESHHOLD) {
      updateTasks.push(() => backend.resolve(backend.getDisplay(index), displayCallback(index)));
      update();
    } else {
      console.error("CAYLON: Got too many display errors for " + index.toString() + ", stopping display updates for element");
      backend.log(backend.CLogLevel.ERROR, "Got too many display errors for " + index.toString() + ", stopping display updates for element");
    }
  }
}

let jsErrors: number = 0;
const JAVASCRIPT_ERROR_ABORT_THRESHHOLD = 16;

function onGetElements() {
  displayErrors = [];
  for (let i = 0; i < items.length; i++) {
    console.log("CAYLON: req display for item #" + i.toString());
    backend.log(backend.CLogLevel.DEBUG, "req display for item #" + i.toString());
    displayErrors.push(0);
    backend.resolve(backend.getDisplay(i), displayCallback(i));
  }
  jsErrors = 0;
  backend.resolve(backend.getJavascriptToRun(), jsCallback());
  register_for_steam_events();
}

const eval2 = eval;

function jsCallback() {
  return (script: backend.CJavascriptResponse) => {
    // register next callback (before running JS, in case that crashes)
    if (jsErrors < JAVASCRIPT_ERROR_ABORT_THRESHHOLD) {
      backend.resolve(backend.getJavascriptToRun(), jsCallback());
    } else {
      console.error("CAYLON: Got too many javascript errors, stopping remote javascript execution");
      backend.log(backend.CLogLevel.ERROR, "Got too many javascript errors, stopping remote javascript execution");
    }
    if (script != null) {
      switch (script.result) {
        case "javascript":
          let toRun = script as backend.CJavascriptResult;
          console.log("CAYLON: Got javascript " + toRun.id.toString(), toRun);
          backend.log(backend.CLogLevel.DEBUG, "Got javascript " + toRun.id.toString());
          try {
            let result = eval2(toRun.raw);
            backend.onJavascriptResult(toRun.id, result);
            jsErrors = 0;
          } catch (err) {
            jsErrors++;
            console.warn("CAYLON: Javascript " + toRun.id.toString() + "failed", err);
            backend.log(backend.CLogLevel.WARN, "Javascript " + toRun.id.toString() + "failed");
          }
          break;
        case "error":
          jsErrors++;
          let err = script as backend.CErrorResult;
          console.warn("CAYLON: Got javascript retrieval error", err);
          backend.log(backend.CLogLevel.WARN, "Got javascript retrieval error");
          break;
        default:
          jsErrors++;
          console.error("CAYLON: Got invalid javascript response", script);
          backend.log(backend.CLogLevel.ERROR, "Got invalid javascript response");
          break;
      }
    } else {
      jsErrors++;
    }
  }
}

// init USDPL WASM frontend
// this is required to interface with the backend
(async () => {
  await backend.initBackend();
  let about_promise = backend.getAbout();
  let elements_promise = backend.getElements();
  about = await about_promise;
  console.log("CAYLON: Got about", about);
  backend.log(backend.CLogLevel.DEBUG, "Got about");
  let result = await elements_promise;
  console.log("CAYLON: Got elements", result);
  backend.log(backend.CLogLevel.DEBUG, "Got elements");
  if (result != null) {
    items = result;
    onGetElements();
  } else {
    console.error("CAYLON: Backend connection failed");
    backend.log(backend.CLogLevel.ERROR, "Backend connection failed");
  }
  backend.resolve(backend.getJavascriptToRun(), jsCallback());
  register_for_steam_events();
})();

const Content: VFC<{ serverAPI: ServerAPI }> = ({}) => {

  const [triggerInternal, updateInternal] = useState<boolean>(false);

  update = () => {
    updateInternal(!triggerInternal);
  }

  function updateIdc(_: any) {
    update();
  }

  // perform tasks (like updating display elements) only while rendering the plugin
  let taskItem = updateTasks.pop();
  while (taskItem != undefined) {
    taskItem();
    taskItem = updateTasks.pop();
  }

  return (
    <PanelSection>
      {items.map(
        (elem, i) => {
          return <PanelSectionRow>{buildHtmlElement(elem, i, updateIdc)}</PanelSectionRow>
        })
      }
      { about != null && buildAbout() }
      <PanelSectionRow>
        <ButtonItem
          layout="below"
          onClick={(_: MouseEvent) => {
            backend.resolve(backend.reload(),
              (reload_items: backend.CElement[]) => {
                items = reload_items;
                console.log("CAYLON: Got elements", reload_items);
                backend.log(backend.CLogLevel.DEBUG, "Got elements for reload");
                if (reload_items != null) {
                  items = reload_items;
                  onGetElements();
                } else {
                  console.error("CAYLON: Backend connection failed");
                  backend.log(backend.CLogLevel.ERROR, "Backend connection failed on reload");
                }
                update();
              });
            backend.resolve(backend.getAbout(),
              (new_about: backend.CAbout) => {
                about = new_about;
                console.log("CAYLON: Got about", about);
                backend.log(backend.CLogLevel.DEBUG, "Got about for reload");
                update();
              });
          }}>
          Reload
        </ButtonItem>
      </PanelSectionRow>
    </PanelSection>
  );
};

function buildHtmlElement(element: backend.CElement, index: number, updateIdc: any) {
  switch (element.element) {
    case "button":
      return buildButton(element as backend.CButton, index, updateIdc);
    case "slider":
      return buildSlider(element as backend.CSlider, index, updateIdc);
    case "toggle":
      return buildToggle(element as backend.CToggle, index, updateIdc);
    case "reading":
      return buildReading(element as backend.CReading, index, updateIdc);
    case "result-display":
      return buildResultDisplay(element as backend.CResultDisplay, index, updateIdc);
    case "event-display":
      return buildEventDisplay(element as backend.CEventDisplay, index, updateIdc);
  }
  console.error("CAYLON: Unsupported element", element);
  backend.log(backend.CLogLevel.ERROR, "Unsupported element " + element.element);
  return <div>Unsupported</div>;
}

function buildButton(element: backend.CButton, index: number, updateIdc: any) {
  return (
    <ButtonItem
      layout="below"
      onClick={() => {backend.resolve(backend.onUpdate(index, null), updateIdc)}}>
      {element.title}
    </ButtonItem>
  );
}

function buildSlider(element: backend.CSlider, index: number, updateIdc: any) {
  const KEY = VALUE_KEY + index.toString();
  if (get_value(KEY) == null) {
    set_value(KEY, element.min);
  }
  return (
    <SliderField
      label={element.title}
      value={get_value(KEY)}
      max={element.max}
      min={element.min}
      showValue={true}
      onChange={(value: number) => {
        backend.resolve(backend.onUpdate(index, value), updateIdc);
        set_value(KEY, value);
      }}
    />
  );
}

function buildToggle(element: backend.CToggle, index: number, updateIdc: any) {
  const KEY = VALUE_KEY + index.toString();
  if (get_value(KEY) == null) {
    set_value(KEY, false);
  }
  return (
    <ToggleField
      checked={get_value(KEY)}
      label={element.title}
      description={element.description!}
      onChange={(value: boolean) => {
        backend.resolve(backend.onUpdate(index, value), updateIdc);
        set_value(KEY, value);
      }}
    />
  );
}

function buildReading(element: backend.CReading, index: number, _updateIdc: any) {
  return (
    <div className={FieldWithSeparator}>
      <div className={gamepadDialogClasses.FieldLabelRow}>
        <div className={gamepadDialogClasses.FieldLabel}>{element.title}</div>
        <div className={gamepadDialogClasses.FieldChildren}>{get_value(DISPLAY_KEY + index.toString())}</div>
      </div>
    </div>
  );
}

function buildResultDisplay(element: backend.CResultDisplay, index: number, _updateIdc: any) {
  return (
    <div className={FieldWithSeparator}>
      <div className={gamepadDialogClasses.FieldLabelRow}>
        <div className={gamepadDialogClasses.FieldLabel}>{element.title}</div>
        <div className={gamepadDialogClasses.FieldChildren}>{get_value(DISPLAY_KEY + index.toString())}</div>
      </div>
    </div>
  );
}

function buildEventDisplay(element: backend.CEventDisplay, index: number, _updateIdc: any) {
  return (
    <div className={FieldWithSeparator}>
      <div className={gamepadDialogClasses.FieldLabelRow}>
        <div className={gamepadDialogClasses.FieldLabel}>{element.title}</div>
        <div className={gamepadDialogClasses.FieldChildren}>{get_value(DISPLAY_KEY + index.toString())}</div>
      </div>
    </div>
  );
}

function buildAbout() {
  if (about == null) {
    return [];
  } else {
    let elements = [
      <div className={staticClasses.PanelSectionTitle}>
        About
      </div>,
      <PanelSectionRow>
        <div className={FieldWithSeparator}>
          <div className={gamepadDialogClasses.FieldLabelRow}>
            <div className={gamepadDialogClasses.FieldLabel}>Name</div>
            <div className={gamepadDialogClasses.FieldChildren}>{about.name}</div>
          </div>
        </div>
      </PanelSectionRow>,
      <PanelSectionRow>
        <div className={FieldWithSeparator}>
          <div className={gamepadDialogClasses.FieldLabelRow}>
            <div className={gamepadDialogClasses.FieldLabel}>Version</div>
            <div className={gamepadDialogClasses.FieldChildren}>{about.version}</div>
          </div>
        </div>
      </PanelSectionRow>,
      <PanelSectionRow>
        <div className={FieldWithSeparator}>
          <div className={gamepadDialogClasses.FieldLabelRow}>
            <div className={gamepadDialogClasses.FieldLabel}>Description</div>
            <div className={gamepadDialogClasses.FieldDescription}>{about.description}</div>
          </div>
        </div>
      </PanelSectionRow>
    ];
    if (about.url != null) {
      elements.push(
        <PanelSectionRow>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>URL</div>
              <div className={gamepadDialogClasses.FieldDescription}>{about.url}</div>
            </div>
          </div>
        </PanelSectionRow>
      );
    }
    if (about.authors.length > 1) {
      let authors = about.authors.map((elem, i) => {
        if (i == about!.authors.length - 1) {
          return <p>{elem}</p>;
        } else {
          return <span>{elem}</span>;
        }
      });
      elements.push(
        <PanelSectionRow>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>Authors</div>
              <div className={gamepadDialogClasses.FieldDescription}>{authors}</div>
            </div>
          </div>
        </PanelSectionRow>
      );
    } else if (about.authors.length == 1) {
      elements.push(
        <PanelSectionRow>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>Author</div>
              <div className={gamepadDialogClasses.FieldDescription}>{about.authors[0]}</div>
            </div>
          </div>
        </PanelSectionRow>
      );
    } else {
      elements.push(
        <PanelSectionRow>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>Author</div>
              <div className={gamepadDialogClasses.FieldDescription}>NGnius</div>
            </div>
          </div>
        </PanelSectionRow>
      );
    }

    if (about.license != null) {
      elements.push(
        <PanelSectionRow>
          <div className={FieldWithSeparator}>
            <div className={gamepadDialogClasses.FieldLabelRow}>
              <div className={gamepadDialogClasses.FieldLabel}>License</div>
              <div className={gamepadDialogClasses.FieldChildren}>{about.license}</div>
            </div>
          </div>
        </PanelSectionRow>
      );
    }
    return elements;
  }
}

export default definePlugin((serverApi: ServerAPI) => {
  return {
    title: <div className={staticClasses.Title}>{about == null? "Caylon": about.name}</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <GiWashingMachine />,
    onDismount() {
      unregister_for_steam_events();
    },
  };
});
