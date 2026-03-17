import { Digits, System, DigitKind, Vector, Matrix } from '~/types';

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

function isDigit(obj: unknown, size: number): obj is Digits {
  if (typeof obj !== 'object' || obj === null) return false;

  const o = obj as Record<string, unknown>;

  if (typeof o.type !== 'string') return false;

  switch (o.type) {
    case 'Explicit':
      return (
        Array.isArray(o.values) && o.values.every((v) => isVector(v, size))
      );

    case 'Canonical':
      return true;

    case 'JCanonical':
      return typeof o.jValue === 'number';

    case 'Dense':
      return true;

    case 'Adjoined':
      return true;

    case 'Symmetric':
      return true;

    case 'Shifted':
      return typeof o.shift === 'number';

    default:
      return false;
  }
}

function isSystem(obj: unknown): obj is Omit<System, 'id'> {
  if (typeof obj !== 'object' || obj === null) return false;

  const o = obj as Record<string, unknown>;
  o.isGns = null;
  o.signature = null;
  o.lastJob = null;

  return (
    typeof o.dimension === 'number' &&
    isSquareMatrix(o.base, o.dimension) &&
    isDigit(o.digits, o.dimension)
  );
}

export async function parseInputFile(
  jsonFile: File,
): Promise<Omit<System, 'id'>> {
  const text = await jsonFile.text();
  let system = JSON.parse(text);

  if (isSystem(system)) {
    return system;
  }

  throw Error('Invalid input file');
}
