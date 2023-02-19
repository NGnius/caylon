import { Component } from "react";
/*import {
  ButtonItem,
  PanelSectionRow,
  SliderField,
  ToggleField,
  Field,
} from "decky-frontend-lib";*/

//import { get_value, set_value } from "usdpl-front";
//import {DISPLAY_KEY, VALUE_KEY} from "../consts";
import { SingleElement } from "./element";
import * as backend from "../backend";

interface ElementsProps {
  items: backend.CElement[];
  displayErrors: number[];
  displayCallback: (index: number) => (ok: boolean) => void;
  schedule: (update: () => void) => void;
};

export class Elements extends Component<ElementsProps, boolean> {

    render() {
        /*const update = () => {
            this.setState((state) => !state);
        };

        const updateIdc = (_: any) => {
            update();
        };*/

        return this.props.items.map(
            (elem, i) => {
                return <SingleElement
                        item={elem}
                        index={i}
                        errors={this.props.displayErrors[i]}
                        callback={this.props.displayCallback(i)}
                        next={this.props.schedule} />;
            }
        );
    }
}
