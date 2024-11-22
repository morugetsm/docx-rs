import { LevelJSON } from "./json";
import {
  createDefaultParagraphProperty,
  ParagraphProperty,
  SpecialIndentKind,
} from "./paragraph-property";
import {
  createDefaultRunProperty,
  RunFonts,
  RunProperty,
} from "./run-property";

export type LevelSuffixType = "nothing" | "tab" | "space";

export class Level {
  id: number;
  start: number;
  format: string;
  text: string;
  jc: string;
  paragraphProperty: ParagraphProperty = createDefaultParagraphProperty();
  runProperty?: RunProperty = createDefaultRunProperty();
  levelSuffix: LevelSuffixType;

  constructor(
    id: number,
    start: number,
    format: string,
    text: string,
    jc: string
  ) {
    this.id = id;
    this.start = start;
    this.format = format;
    this.text = text;
    this.jc = jc;
    this.levelSuffix = "tab";
  }

  indent(
    left: number,
    specialIndentKind?: SpecialIndentKind,
    specialIndentSize?: number
  ) {
    this.paragraphProperty.indent = {
      left,
      specialIndentKind,
      specialIndentSize,
    };
    return this;
  }

  suffix(s: LevelSuffixType) {
    this.levelSuffix = s;
    return this;
  }

  size(size: number) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.size(size);
    return this;
  }

  color(color: string) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.color(color);
    return this;
  }

  highlight(color: string) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.highlight(color);
    return this;
  }

  bold() {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.bold();
    return this;
  }

  italic() {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.italic();
    return this;
  }

  underline(type: string) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.underline(type);
    return this;
  }

  vanish() {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.vanish();
    return this;
  }

  fonts(fonts: RunFonts) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.fonts(fonts);
    return this;
  }

  characterSpacing(characterSpacing: number) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.spacing(characterSpacing);
    return this;
  }
}

export class LevelOverride {
  level: number;
  startOverride: number | null = null;
  levelOverride: LevelJSON | null = null;

  constructor(level: number) {
    this.level = level;
  }

  overrideStart(start: number) {
    this.startOverride = start;
    return this;
  }

  overrideLevel(level: LevelJSON) {
    this.levelOverride = level;
    return this;
  }
}
