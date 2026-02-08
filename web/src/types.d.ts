// ['Custom', 'Canonical', 'J-Canonical', 'Dense', 'Adjoined', 'Symmetric', 'Shifted']);

export type User = {
  id: number,
  userName: string
}

export type Vector = number[];

export type BaseJobData = {
  dimension: number;
  base: number[][];
}
export type CustomJobData = BaseJobData & {
  count: number;
  digits: Vector[];
}
export type CanonicalJobData = BaseJobData & {}
export type JCanonicalJobData = BaseJobData & {
  jValue: number;
}
export type DensJobData = BaseJobData & {}
export type AdjoinedJobData = BaseJobData & {}
export type SymmetricJobData = BaseJobData & {}
export type ShiftedJobData = BaseJobData & {
  shift: number;
}

export type Job = CustomJobData |
  CanonicalJobData | 
  JCanonicalJobData |
  DensJobData |
  AdjoinedJobData |
  SymmetricJobData |
  ShiftedJobData

export type BaseJobResults = {
  decision: boolean;
  walk: Vector[];
  classification: Vector[][]; // List of loops
}

