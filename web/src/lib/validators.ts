import { DigitType, JobType, Norm } from '~/types';

export function validateInputAsFile(input: FormDataEntryValue | null): File {
  if (!input || typeof input !== 'object') {
    throw new Error('No file provided');
  }
  if (input.type !== 'application/json') {
    throw new Error('File has to be JSON');
  }
  if (!input.text || typeof input.text !== 'function') {
    throw new Error('File has to have content');
  }
  return input;
}

export function validateInputAsInteger(
  input: FormDataEntryValue | null,
  entityName: string,
): number {
  if (!input || typeof input !== 'string') {
    throw new Error(`No ${entityName} provided`);
  }
  const parsed: number = parseInt(input);
  if (!Number.isInteger(parsed)) {
    throw new Error(`${entityName} has to be an integer number`);
  }
  return parsed;
}

export function validateInputAsNorm(
  input: FormDataEntryValue | null,
  entityName: string,
): Norm {
  if (!input || typeof input !== 'string') {
    throw new Error(`No ${entityName} provided`);
  }
  if (input != 'Infinite' && input != 'L1' && input != 'L2') {
    throw new Error('Invalid norm value');
  }
  return input;
}

export function validateInputAsJobType(
  input: FormDataEntryValue | null,
  entityName: string,
): JobType {
  if (!input || typeof input !== 'string') {
    throw new Error(`No ${entityName} provided`);
  }

  if (input != 'Walk' && input != 'Decision' && input != 'Classification') {
    throw new Error(`Invalid ${entityName} value`);
  }
  return input;
}

export function validateInputAsVector(
  input: FormDataEntryValue | null,
  dim: number,
  entityName: string,
): number[] {
  if (!input || typeof input !== 'string') {
    throw new Error(`No ${entityName} provided`);
  }
  const values = input.split(' ').map((s) => parseInt(s));
  if (values.length == dim && values.every((v) => Number.isInteger(v))) {
    return values;
  } else {
    throw new Error(`Invalid ${entityName} values`);
  }
}

export function validateInputAsString(
  input: FormDataEntryValue | null,
  entityName: string,
): string {
  if (!input || typeof input !== 'string' || input.length == 0) {
    throw new Error(`No ${entityName} provided`);
  }
  return input;
}

export function validateStringAsName(name: string, entityName: string): string {
  if (name.length < 3) {
    throw new Error(`${entityName} is too short. Min length is 3.`);
  }
  if (name.length > 32) {
    throw new Error(`${entityName} is too long. Max length is 32.`);
  }
  if (!/^[A-Za-z0-9 _\\-]+$/.test(name)) {
    throw new Error(
      `${entityName} must contain only alphanumeric, underscore, hyphen and space characters.`,
    );
  }
  return name;
}

export function validateInputAsName(
  input: FormDataEntryValue | null,
  entityName: string,
): string {
  const name = validateInputAsString(input, entityName);
  return validateStringAsName(name, entityName);
}

export function validateUsername(
  input: FormDataEntryValue | null,
  entityName: string,
): string {
  const userName = validateInputAsString(input, entityName);
  if (userName.length < 3) {
    throw Error(`${entityName} is too short. Min length is 3.`);
  }
  if (userName.length > 32) {
    throw Error(`${entityName} is too long. Max length is 32.`);
  }

  return userName;
}

function validateIntegerAsPositive(n: number, entityName: string): number {
  if (n <= 0) {
    throw new Error(`${entityName} must be a positive integer`);
  }
  return n;
}

export function validateNumberAsPositiveInteger(
  n: number,
  entityName: string,
): number {
  if (!Number.isInteger(n)) {
    throw new Error(`${entityName} must be an integer.`);
  }
  return validateIntegerAsPositive(n, entityName);
}

export function validateInputAsPositiveInteger(
  input: FormDataEntryValue | null,
  entityName: string,
): number {
  const n = validateInputAsInteger(input, entityName);
  return validateIntegerAsPositive(n, entityName);
}

export function digitNeedsParam(digitType: DigitType): boolean {
  switch (digitType) {
    case 'Canonical':
    case 'Explicit':
    case 'Symmetric':
    case 'Adjoined':
      return false;
    case 'JCanonical':
    case 'JSymmetric':
    case 'Shifted':
      return true;
  }
}
function validateStringAsIntegerVector(input: string): number[] {
  input = input.trim();
  if (input.length == 0) {
    return [];
  }
  const flattenedString = input.replaceAll('\n', ' ');
  const vectorValueRegex = /^-?\d+(?: -?\d+)*$/g;
  const regmatch = vectorValueRegex.test(flattenedString);
  if (!regmatch) {
    throw new Error(
      'Input format is invalid. Requires integers separated by spaces or new lines.',
    );
  }
  const values = flattenedString.split(' ').map((s) => parseInt(s));

  if (values.find(isNaN)) {
    throw new Error('The vector elements should be integers');
  }
  return values;
}

export function validateStringAsExplicitDigits(
  input: string,
  dim: number,
): number[][] {
  const numbers = validateStringAsIntegerVector(input);
  if (!numbers.length) {
    return [];
  }
  let splitArr: number[][] = [];
  while (numbers.length) {
    splitArr.push(numbers.splice(0, dim));
  }
  if (!splitArr.every((v) => v.length == dim)) {
    throw new Error(`All vectors must be of length of the dimension (${dim})`);
  }
  return splitArr;
}

export function validateInputAsExplicitDigits(
  input: FormDataEntryValue | null,
  dim: number,
): number[][] {
  const rawInput = validateInputAsString(input, 'digits');
  return validateStringAsExplicitDigits(rawInput, dim);
}

export function validateStringAsFlatMatrix(
  input: string,
  dim: number,
): number[][] {
  const numbers = validateStringAsIntegerVector(input);
  if (numbers.length != dim * dim) {
    throw new Error(`Size of matrix must be ${dim}x${dim}.`);
  }
  let rows: number[][] = [];
  while (numbers.length) {
    rows.push(numbers.splice(0, dim));
  }
  return rows;
}

export function validateInputAsFlatMatrix(
  input: FormDataEntryValue | null,
  dim: number,
): number[][] {
  const rawInput = validateInputAsString(input, 'base');
  return validateStringAsFlatMatrix(rawInput, dim);
}

export function validateStringAsDigitType(type: string): DigitType {
  const types: string[] = [
    'Adjoined',
    'Canonical',
    'JCanonical',
    'Explicit',
    'Symmetric',
    'JSymmetric',
    'Shifted',
  ];
  if (!types.includes(type)) {
    throw new Error(`Invalid digit type value`);
  }
  return type as DigitType;
}

export function validateInputAsDigitType(
  input: FormDataEntryValue | null,
): DigitType {
  const type = validateInputAsString(input, 'digit type');
  return validateStringAsDigitType(type);
}

export function validateNumberAsJValue(n: number, dim: number): number {
  const i = validateNumberAsPositiveInteger(n, 'jValue');
  if (i >= dim) {
    throw new Error(`jValue has to be smaller than dimension (${dim}).`);
  }
  return n;
}

export function validateInputAsJValue(
  input: FormDataEntryValue | null,
  entityName: string,
  dim: number,
): number {
  const n = validateInputAsPositiveInteger(input, entityName);
  if (n >= dim) {
    throw new Error(`${entityName} has to be smaller than dimension.`);
  }
  return n;
}

function isIntegerArr(arr: unknown[]): arr is number[] {
  return arr.every(Number.isInteger);
}

function validateArrayAsGridPoint(
  arr: unknown[],
  entityName: string,
  dim: number,
): number[] {
  if (!isIntegerArr(arr)) {
    throw new Error(`${entityName} must only contain integers`);
  }

  if (arr.length !== dim) {
    throw new Error(
      `${entityName} must be of the same length as dimension (${dim})`,
    );
  }

  return arr;
}

function isGridPointArray(
  arr: unknown[],
  entityName: string,
  dim: number,
): arr is number[][] {
  return arr.every(
    (e) => Array.isArray(e) && validateArrayAsGridPoint(e, entityName, dim),
  );
}

export function validateArrayAsGridPointArray(
  m: unknown[],
  dim: number,
  entityName: string,
  subEntityName: string,
): number[][] {
  if (!isGridPointArray(m, subEntityName, dim)) {
    throw new Error(`${entityName} must be an array of arrays.`);
  }
  if (m.length == 0) {
    throw new Error(`${entityName} must not be empty.`);
  }
  return m;
}

export function validateArrayAsSquareMatrix(
  m: unknown[],
  dim: number,
  entityName: string,
) {
  const matrix = validateArrayAsGridPointArray(
    m,
    dim,
    entityName,
    entityName + ' row',
  );
  if (matrix.length != dim) {
    throw new Error(
      `${entityName} must have same same number of rows as dimension (${dim})`,
    );
  }

  return matrix;
}

export function isObject(obj: unknown): obj is Object {
  if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
    throw new Error('File is not a valid JSON object.');
  }
  return true;
}
