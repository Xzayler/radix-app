import { JobType, Norm } from '~/types';

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

export function validateInputAsNumber(
  input: FormDataEntryValue | null,
  entityName: string,
): number {
  if (!input || typeof input !== 'string') {
    throw new Error(`No ${entityName} provided`);
  }
  const systemId: number = parseInt(input);
  if (isNaN(systemId)) {
    throw new Error(`${entityName} has to be an integer number`);
  }
  return systemId;
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
  if (values.length == dim && values.every((v) => !isNaN(v))) {
    return values;
  } else {
    throw new Error(`Invalid ${entityName}values`);
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

export function validateFileName(
  input: FormDataEntryValue | null,
  entityName: string,
): string {
  const name = validateInputAsString(input, entityName);
  if (name.length > 32) {
    throw Error(`${entityName} is too long. Max length is 32.`);
  }
  return name;
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
