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
} from "decky-frontend-lib";
import { VFC, useState } from "react";
import { GiWashingMachine } from "react-icons/gi";

//import { set_value } from "usdpl-front";
import * as backend from "./backend";
import {register_for_steam_events, unregister_for_steam_events} from "./steam_events";
//import {DISPLAY_KEY} from "./consts";
import {Elements} from "./components/elements";
import {About} from "./components/about";

let items: backend.CElement[] = [];
let about: backend.CAbout | null = null;

let update = () => {};

let updateTasks: (() => void)[] = [];

function scheduleCall(cb: () => void) {
  updateTasks.push(cb);
}

let jsErrors: number = 0;
const JAVASCRIPT_ERROR_ABORT_THRESHOLD = 16;

let displayErrors: number[] = [];

function displayCallback(index: number) {
  return (ok: boolean) => {
    if (ok) {
      displayErrors[index] = 0;
    } else {
      displayErrors[index]++;
    }
  }
}

function onGetElements() {
  displayErrors = [];
  for (let i = 0; i < items.length; i++) {
    console.log("CAYLON: req display for item #" + i.toString());
    backend.log(backend.CLogLevel.DEBUG, "req display for item #" + i.toString());
    displayErrors.push(0);
  }
  jsErrors = 0;
  backend.resolve(backend.getJavascriptToRun(), jsCallback());
  register_for_steam_events();
}

const eval2 = eval;

function jsCallback() {
  return (script: backend.CJavascriptResponse) => {
    // register next callback (before running JS, in case that crashes)
    if (jsErrors < JAVASCRIPT_ERROR_ABORT_THRESHOLD) {
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

  // perform tasks (like updating display elements) only while rendering the plugin
  let taskItem = updateTasks.pop();
  while (taskItem != undefined) {
    taskItem();
    taskItem = updateTasks.pop();
  }

  return (
    <PanelSection>
      <Elements
        items={items}
        displayErrors={displayErrors}
        displayCallback={displayCallback}
        schedule={scheduleCall} />
      <About about={about}/>
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
