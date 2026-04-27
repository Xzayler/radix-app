import {
  Digits,
  System,
  DigitType,
  Vector,
  Matrix,
  NewSystem,
  ExplicitDigits,
  ShiftedDigits,
} from '~/types';
import {
  isObject,
  validateArrayAsGridPointArray,
  validateArrayAsSquareMatrix,
  validateInputAsJValue,
  validateInputAsName,
  validateNumberAsJValue,
  validateNumberAsPositiveInteger,
  validateStringAsDigitType,
  validateStringAsName,
} from '../validators';

function isVector(v: unknown, size: number): v is Vector {
  return (
    Array.isArray(v) &&
    v.every((n) => typeof n === 'number') &&
    v.length == size
  );
}

function isSquareMatrix(m: unknown, size: number): m is Matrix {
  return Array.isArray(m) && m.every((v) => isVector(v, size));
}

function isDigit(obj: unknown, dim: number): obj is Digits {
  if (typeof obj !== 'object' || obj === null) return false;

  const o = obj as Record<string, unknown>;

  if (typeof o.type !== 'string') return false;

  switch (o.type) {
    case 'Explicit':
      return Array.isArray(o.values) && o.values.every((v) => isVector(v, dim));

    case 'Canonical':
      return true;

    case 'JCanonical':
      return typeof o.jValue === 'number';

    case 'Adjoint':
      return true;

    case 'Symmetric':
      return true;

    case 'JSymmetric':
      return typeof o.jValue === 'number';

    case 'Shifted':
      return typeof o.shift === 'number';

    default:
      return false;
  }
}

function parseDigits(obj: Object, dim: number): Digits {
  if (!('type' in obj) || typeof obj.type !== 'string') {
    throw new Error('Digits must include a "type" string property.');
  }
  const type = validateStringAsDigitType(obj.type);

  switch (type) {
    case 'Explicit':
      if (!('values' in obj) || !Array.isArray(obj.values)) {
        throw new Error(
          'Digits must include a "values" array property if "type" is "Explicit".',
        );
      }
      const values = validateArrayAsGridPointArray(
        obj.values,
        dim,
        'values',
        'value',
      );
      return { type, values } as ExplicitDigits;

    case 'JCanonical':
    case 'JSymmetric':
      if (!('jValue' in obj) || typeof obj.jValue != 'number') {
        throw new Error(
          `Digits must include a "jValue" array property if "type" is ${type}.`,
        );
      }
      const jValue = validateNumberAsJValue(obj.jValue, dim);
      return { type, jValue };
    case 'Shifted':
      if (!('shift' in obj) || typeof obj.shift != 'number') {
        throw new Error(
          `Digits must include a "jValue" array property if "type" is ${type}.`,
        );
      }
      const shift = validateNumberAsPositiveInteger(obj.shift, 'shift');
      return { type, shift } as ShiftedDigits;
    case 'Adjoint':
    case 'Canonical':
    case 'Symmetric':
      return { type } as Digits;
    default:
      throw new Error('Invalid digit type.');
  }
}

function parseSystem(obj: unknown): NewSystem {
  if (!isObject(obj)) {
    throw new Error('File is not a valid JSON object.');
  }

  if (!('name' in obj) || typeof obj.name !== 'string') {
    throw new Error('JSON must include a "name" string property.');
  }
  const name = validateStringAsName(obj.name, 'name');

  if (!('dimension' in obj) || typeof obj.dimension !== 'number') {
    throw new Error('JSON must include a "dimension" number property.');
  }
  const dimension = validateNumberAsPositiveInteger(obj.dimension, 'dimension');

  if (!('base' in obj) || !Array.isArray(obj.base)) {
    throw new Error('JSON must include a "base" number array property.');
  }
  const base = validateArrayAsSquareMatrix(obj.base, dimension, 'base');

  if (!('digits' in obj) || !isObject(obj.digits)) {
    throw new Error('JSON must include a "digits" object property.');
  }
  const digits = parseDigits(obj.digits, dimension);

  return {
    name,
    dimension,
    base,
    digits,
  };
}

export async function parseInputFile(jsonFile: File): Promise<NewSystem> {
  const text = await jsonFile.text();
  return parseSystem(JSON.parse(text));
}
