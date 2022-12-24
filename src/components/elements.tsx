import { Component, useState } from "react";
import {
  ButtonItem,
  PanelSectionRow,
  SliderField,
  ToggleField,
  Field,
} from "decky-frontend-lib";

import { get_value, set_value } from "usdpl-front";
import {DISPLAY_KEY, VALUE_KEY} from "../consts";
import * as backend from "../backend";

export class Elements extends Component<{items: backend.CElement[]}> {

    render() {
        const [triggerInternal, updateInternal] = useState<boolean>(false);

        const update = () => {
            updateInternal(!triggerInternal);
        }

        function updateIdc(_: any) {
            update();
        }

        return this.props.items.map(
            (elem, i) => {
                return (<PanelSectionRow>{buildHtmlElement(elem, i, updateIdc)}</PanelSectionRow>);
            }
        );
    }
}

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
  return (<div>Unsupported</div>);
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
    <Field label={element.title}>
      {get_value(DISPLAY_KEY + index.toString())}
    </Field>
  );
}

function buildResultDisplay(element: backend.CResultDisplay, index: number, _updateIdc: any) {
  return (
    <Field label={element.title}>
      {get_value(DISPLAY_KEY + index.toString())}
    </Field>
  );
}

function buildEventDisplay(element: backend.CEventDisplay, index: number, _updateIdc: any) {
  return (
    <Field label={element.title}>
      {get_value(DISPLAY_KEY + index.toString())}
    </Field>
  );
}
