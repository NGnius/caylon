import { Component } from "react";
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

const DISPLAY_ERROR_ABORT_THRESHOLD = 8;

interface ElementProps {
    item: backend.CElement;
    index: number;
    errors: number;
    callback: (ok: boolean) => void;
    next: (update: () => void) => void;
};

export class SingleElement extends Component<ElementProps, boolean> {
    constructor(props: ElementProps) {
        super(props);
        this.state = true;
        // setup first callback
        const update = () => {
            this.setState((state) => !state);
        };
        if (this.props.errors < DISPLAY_ERROR_ABORT_THRESHOLD) {
            backend.resolve(backend.getDisplay(this.props.index), this.displayCallback(update));
        }

    }

    render() {

        const update = () => {
            this.setState((state) => !state);
        };

        const updateIdc = (_: any) => {
            update();
        };

        return (
            <PanelSectionRow>
                {buildHtmlElement(this.props.item, this.props.index, updateIdc)}
            </PanelSectionRow>
        );
    }

    displayCallback(update: () => void) {
        return (newVal: backend.CDisplayResponse) => {
            const index = this.props.index;
            if (newVal != null) {
                switch (newVal.result) {
                    case "value":
                        this.props.callback(true);
                        let val = newVal as backend.CValueResult;
                        console.log("CAYLON: Got display for " + index.toString(), val);
                        backend.log(backend.CLogLevel.DEBUG, "Got display for " + index.toString());
                        set_value(DISPLAY_KEY + index.toString(), val.value);
                        break;
                    case "error":
                        this.props.callback(false);
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
                this.props.callback(false);
                console.warn("CAYLON: Ignoring null display result for " + index.toString());
                backend.log(backend.CLogLevel.WARN, "Ignoring null display result for " + index.toString());
            }
            if (this.props.errors < DISPLAY_ERROR_ABORT_THRESHOLD) {
                backend.resolve(backend.getDisplay(this.props.index), this.displayCallback(update));
                backend.log(backend.CLogLevel.INFO, "Resubscribing for display " + this.props.index.toString());
            } else {
                console.error("CAYLON: Got too many display errors for " + this.props.index.toString() + ", stopping display updates for element");
                backend.log(backend.CLogLevel.ERROR, "Got too many display errors for " + this.props.index.toString() + ", stopping display updates for element");
            }
            update();
        }
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
  console.error("CAYLON: Unrecognized element", element);
  backend.log(backend.CLogLevel.ERROR, "Unrecognized element " + element.element);
  return (<div>Unknown</div>);
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
    <Field label={element.title} focusable={true}>
      {get_value(DISPLAY_KEY + index.toString())}
    </Field>
  );
}

function buildResultDisplay(element: backend.CResultDisplay, index: number, _updateIdc: any) {
  return (
    <Field label={element.title} focusable={true}>
      {get_value(DISPLAY_KEY + index.toString())}
    </Field>
  );
}

function buildEventDisplay(element: backend.CEventDisplay, index: number, _updateIdc: any) {
  return (
    <Field label={element.title} focusable={true}>
      {get_value(DISPLAY_KEY + index.toString())}
    </Field>
  );
}
