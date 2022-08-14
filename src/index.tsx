import {
  ButtonItem,
  definePlugin,
  DialogButton,
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

import { call_backend } from "usdpl-front";
import * as backend from "./backend";

// interface AddMethodArgs {
//   left: number;
//   right: number;
// }

const FieldWithSeparator = joinClassNames(gamepadDialogClasses.Field, gamepadDialogClasses.WithBottomSeparatorStandard);

let items: backend.CElement[] = [];

const Content: VFC<{ serverAPI: ServerAPI }> = ({}) => {
  // const [result, setResult] = useState<number | undefined>();

  // const onClick = async () => {
  //   const result = await serverAPI.callPluginMethod<AddMethodArgs, number>(
  //     "add",
  //     {
  //       left: 2,
  //       right: 2,
  //     }
  //   );
  //   if (result.success) {
  //     setResult(result.result);
  //   }
  // };

  const [triggerInternal, updateInternal] = useState<boolean>(false);

  function update() {
    updateInternal(!triggerInternal);
  }

  function updateIdc(_: any) {
    update();
  }
  
  // call hello callback on backend
  (async () => {
    let response = await call_backend("hello", []);
    console.log("Backend says:", response);
  })();

  return (
    <PanelSection title="Panel Section">
      {items.map(
        (elem, i) => {
          return <PanelSectionRow>{buildHtmlElement(elem, i, updateIdc)}</PanelSectionRow>
        })
      }
    </PanelSection>
  );
};

const DeckyPluginRouterTest: VFC = () => {
  return (
    <div style={{ marginTop: "50px", color: "white" }}>
      Hello World!
      <DialogButton onClick={() => {}}>
        Go to Store
      </DialogButton>
    </div>
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
  }
  return "Unsupported";
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
  return (
    <SliderField
      label={element.title}
      value={element.min}
      max={element.max}
      min={element.min}
      showValue={true}
      onChange={(value: number) => {
        backend.resolve(backend.onUpdate(index, value), updateIdc)
      }}
    />
  );
}

function buildToggle(element: backend.CToggle, index: number, updateIdc: any) {
  return (
    <ToggleField
      checked={false}
      label={element.title}
      description={element.description!}
      onChange={(value: boolean) => {
        backend.resolve(backend.onUpdate(index, value), updateIdc)
      }}
    />
  );
}

function buildReading(element: backend.CReading, _index: number, _updateIdc: any) {
  return (
    <div className={FieldWithSeparator}>
      <div className={gamepadDialogClasses.FieldLabelRow}>
        <div className={gamepadDialogClasses.FieldLabel}>{element.title}</div>
        <div className={gamepadDialogClasses.FieldChildren}>{"idk"}</div>
      </div>
    </div>
  );
}

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute("/decky-plugin-test", DeckyPluginRouterTest, {
    exact: true,
  });
  
  // init USDPL WASM frontend
  // this is required to interface with the backend
  (async () => {
    await backend.initBackend();
    items = await backend.getElements();
  })();

  return {
    title: <div className={staticClasses.Title}>Example Plugin</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <GiWashingMachine />,
    onDismount() {
      serverApi.routerHook.removeRoute("/decky-plugin-test");
    },
  };
});
