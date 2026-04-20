export type ColumnType =
  | 'dimension'
  | 'name'
  | 'base'
  | 'digitType'
  | 'digits'
  | 'isGns'
  | 'signature'
  | 'lastJob'
  | 'operations';

export type SystemColumn = {
  type: ColumnType;
  label: string;
};

export const columns: SystemColumn[] = [
  { type: 'dimension', label: 'Dim' },
  { type: 'name', label: 'Name' },
  { type: 'base', label: 'Base' },
  { type: 'digitType', label: 'Digit Types' },
  { type: 'digits', label: 'Digits' },
  { type: 'isGns', label: 'GNS' },
  { type: 'signature', label: 'Signature' },
  { type: 'lastJob', label: 'Last Job' },
  { type: 'operations', label: '' },
];
