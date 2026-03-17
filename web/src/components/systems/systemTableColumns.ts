export type ColumnType =
  | 'dimension'
  | 'base'
  | 'digitType'
  | 'digits'
  | 'isGns'
  | 'signature'
  | 'lastJob';

export type SystemColumn = {
  type: ColumnType;
  label: string;
};

export const columns: SystemColumn[] = [
  { type: 'dimension', label: 'Dim' },
  { type: 'base', label: 'Base' },
  { type: 'digitType', label: 'Digit Types' },
  { type: 'digits', label: 'Digits' },
  { type: 'isGns', label: 'GNS' },
  { type: 'signature', label: 'Signature' },
  { type: 'lastJob', label: 'Last Job' },
];
