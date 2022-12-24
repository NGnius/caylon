import {Component} from "react";
import {
  Field,
  PanelSectionRow,
  staticClasses,
} from "decky-frontend-lib";
import * as backend from "../backend";

export class About extends Component<{about: backend.CAbout | null}> {
    render() {
        return buildAbout(this.props.about);
    }
}

function buildAbout(about: backend.CAbout | null) {
  if (about == null) {
    return [];
  } else {
    let elements = [
      <div className={staticClasses.PanelSectionTitle}>
        About
      </div>,
      <PanelSectionRow>
        <Field label="Name">
          {about.name}
        </Field>
      </PanelSectionRow>,
      <PanelSectionRow>
        <Field label="Version">
          {about.version}
        </Field>
      </PanelSectionRow>,
      <PanelSectionRow>
        <Field label="Description">
          {about.description}
        </Field>
      </PanelSectionRow>
    ];
    if (about.url != null) {
      elements.push(
        <PanelSectionRow>
          <Field label="URL">
            {about.url}
          </Field>
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
          <Field label="Authors">
            {authors}
          </Field>
        </PanelSectionRow>
      );
    } else if (about.authors.length == 1) {
      elements.push(
        <PanelSectionRow>
          <Field label="Author">
            {about.authors[0]}
          </Field>
        </PanelSectionRow>
      );
    } else {
      elements.push(
        <PanelSectionRow>
          <Field label="Author">
            NGnius
          </Field>
        </PanelSectionRow>
      );
    }

    if (about.license != null) {
      elements.push(
        <PanelSectionRow>
          <Field label="License">
            {about.license}
          </Field>
        </PanelSectionRow>
      );
    }
    return elements;
  }
}
