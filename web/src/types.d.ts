// ['Custom', 'Canonical', 'J-Canonical', 'Dense', 'Adjoined', 'Symmetric', 'Shifted']);

export type User = {
  id: number;
  userName: string;
};

export type Vector = number[];
export type Matrix = number[][];

export type DigitKind =
  | 'Explicit'
  | 'Canonical'
  | 'JCanonical'
  | 'Dense'
  | 'Adjoined'
  | 'Symmetric'
  | 'Shifted';

export type BaseDigit = {
  type: DigitKind;
};

export type ExplicitDigits = BaseDigit & {
  type: 'Explicit';
  values: Vector[];
};
export type CanonicalDigits = BaseDigit & {
  type: 'Canonical';
};
export type JCanonicalDigits = BaseDigit & {
  type: 'JCanonical';
  jValue: number;
};
export type DenseDigits = BaseDigit & {
  type: 'Dense';
};
export type AdjoinedDigits = BaseDigit & {
  type: 'Adjoined';
};
export type SymmetricDigits = BaseDigit & {
  type: 'Symmetric';
};
export type ShiftedDigits = BaseDigit & {
  type: 'Shifted';
  shift: number;
};

export type Digits =
  | ExplicitDigits
  | CanonicalDigits
  | JCanonicalDigits
  | DenseDigits
  | AdjoinedDigits
  | SymmetricDigits
  | ShiftedDigits;

export type System = {
  id: number;
  dimension: number;
  base: Matrix;
  digits: Digits;
  isGns?: boolean;
  signature?: number[];
};
