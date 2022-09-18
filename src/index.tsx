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

const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

const DISPLAY_KEY = "display";
const VALUE_KEY = "value";

let items: backend.CElement[] = [];
let about: backend.CAbout | null = null;

let update = () => {};

let updateTasks: (() => void)[] = [];

function displayCallback(index: number) {
  return (newVal: backend.CDisplayResponse) => {
    if (newVal != null) {
      switch (newVal.result) {
        case "value":
          let val = newVal as backend.CValueResult;
          console.log("KAYLON: Got display for " + index.toString(), val);
          set_value(DISPLAY_KEY + index.toString(), val.value);
          break;
        case "error":
          let err = newVal as backend.CErrorResult;
          console.warn("KAYLON: Got display error for " + index.toString(), err);
          break;
        default:
          console.error("KAYLON: Got invalid display response for " + index.toString(), newVal);
          break;
      }
    } else {
      console.warn("KAYLON: Ignoring null display result for " + index.toString());
    }
    updateTasks.push(() => backend.resolve(backend.getDisplay(index), displayCallback(index)));
    update();
  }
}

function onGetElements() {
  for (let i = 0; i < items.length; i++) {
    console.log("KAYLON: req display for item #" + i.toString());
    backend.resolve(backend.getDisplay(i), displayCallback(i));
  }
  backend.resolve(backend.getJavascriptToRun(), jsCallback());
}

const eval2 = eval;

function jsCallback() {
  return (script: backend.CJavascriptResponse) => {
    // register next callback (before running JS, in case that crashes)
    backend.resolve(backend.getJavascriptToRun(), jsCallback());
    if (script != null) {
      switch (script.result) {
        case "javascript":
          let toRun = script as backend.CJavascriptResult;
          console.log("KAYLON: Got javascript " + toRun.id.toString(), toRun);
          let result = eval2(toRun.raw);
          backend.onJavascriptResult(toRun.id, result);
          break;
        case "error":
          let err = script as backend.CErrorResult;
          console.warn("KAYLON: Got javascript retrieval error", err);
          break;
        default:
          console.error("KAYLON: Got invalid javascript response", script);
          break;
      }
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
  console.log("KAYLON: got about", about);
  let result = await elements_promise;
  console.log("KAYLON: got elements", result);
  if (result != null) {
    items = result;
    onGetElements();
  } else {
    console.warn("KAYLON: backend connection failed");
  }
  backend.resolve(backend.getJavascriptToRun(), jsCallback());
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
                console.log("KAYLON: got elements", reload_items);
                if (reload_items != null) {
                  items = reload_items;
                  onGetElements();
                } else {
                  console.warn("KAYLON: backend connection failed");
                }
                update();
              });
            backend.resolve(backend.getAbout(),
              (new_about: backend.CAbout) => {
                about = new_about;
                console.log("KAYLON: got about", about);
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
  }
  console.error("KAYLON: Unsupported element", element);
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
    title: <div className={staticClasses.Title}>{about == null? "Kaylon": about.name}</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <GiWashingMachine />,
    onDismount() {
      //serverApi.routerHook.removeRoute("/decky-plugin-test");
    },
  };
});
