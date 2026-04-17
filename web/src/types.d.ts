export type User = {
  id: number;
  userName: string;
};

export type Vector = number[];
export type Matrix = number[][];

export type Norm = 'Infinite' | 'L1' | 'L2';

export type DigitKind =
  | 'Explicit'
  | 'Canonical'
  | 'JCanonical'
  | 'Dense'
  | 'Adjoined'
  | 'Symmetric'
  | 'JSymmetric'
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
  normType: Norm;
};
export type AdjoinedDigits = BaseDigit & {
  type: 'Adjoined';
};
export type SymmetricDigits = BaseDigit & {
  type: 'Symmetric';
};
export type JSymmetricDigits = BaseDigit & {
  type: 'JSymmetric';
  jValue: number;
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
  | JSymmetricDigits
  | ShiftedDigits;

export type System = {
  id: number;
  dimension: number;
  base: Matrix;
  digits: Digits;
  isGns: boolean | null;
  signature: number[] | null;
  lastJob: Date | null;
  isFavourited: boolean;
};

export type JobStatus = 'Pending' | 'Running' | 'Succeeded' | 'Failed';
export type JobType = 'Walk' | 'Decision' | 'Classification';

export type Job = {
  id: number;
  userId: number;
  systemId: number;
  status: JobStatus;
  jobType: JobType;
  norm: Norm;
  walkFrom?: number[];
  outputUri?: string;
  createdAt?: Date;
  startedAt?: Date;
  finishedAt?: Date;
  error?: string;
};

export type NewSystem = Omit<System, 'id'>;
export type NewJob = Omit<
  Job,
  'id' | 'outputUri' | 'createdAt' | 'startedAt' | 'finishedAt'
>;
