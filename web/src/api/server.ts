'use server';
import {
  login as li,
  getCurrentUser as gcu,
  logout as lo,
  register as rg,
  getLoggedInUser,
} from '~/lib/auth';
import {
  getSystemById as gsId,
  insertSystem,
  getSystems as gs,
  getJobs as gj,
  insertJob,
  favouriteSystem as fs,
  unFavouriteSystem as ufs,
  SystemsFilter,
  JobsFilter,
} from '~/lib/db/operations';
import { parseInputFile } from '~/lib/utils/fileParser';
import {
  validateInputAsName,
  validateInputAsFile,
  validateInputAsJobType,
  validateInputAsNorm,
  validateInputAsInteger,
  validateInputAsVector,
  validateInputAsPositiveInteger,
  validateInputAsFlatMatrix,
  validateInputAsDigitType,
  validateInputAsExplicitDigits,
  validateInputAsJValue,
} from '~/lib/validators';
import { getDownloadUrl as gdu } from '~/lib/minio/adapter';
import { Digits, NewJob, NewSystem, System } from '~/types';

// Auth
export const login = li;
export const getCurrentUser = gcu;
export const logout = lo;
export const register = rg;

// Db Entity operations
export const getSystemById = async (systemId: number) => {
  const user = await getLoggedInUser();

  return await dbErrorWrapper(
    () => gsId(systemId, user.id),
    `Error getting system with id ${systemId} job`,
  );
};
export const getSystems = async (params: SystemsFilter) => {
  const user = await getLoggedInUser();

  return await dbErrorWrapper(
    () => gs(params, user.id),
    'Error listing systems',
  );
};
export const uploadSystemFromFile = async (
  formData: FormData,
): Promise<System> => {
  const user = await getLoggedInUser();

  const file = validateInputAsFile(formData.get('input-file'));
  const inputData = await parseInputFile(file);

  return await dbErrorWrapper(
    () => insertSystem(inputData, user.id),
    'Error uploading system',
  );
};

export const uploadSystemFromForm = async (formData: FormData) => {
  const user = await getLoggedInUser();

  const name = validateInputAsName(formData.get('name'), 'name');
  const dim = validateInputAsPositiveInteger(formData.get('dim'), 'dim');
  const base = validateInputAsFlatMatrix(formData.get('base'), dim);
  const digitType = validateInputAsDigitType(formData.get('dtype'));
  let digits: Digits;
  if (digitType == 'Explicit') {
    const explDigits = validateInputAsExplicitDigits(
      formData.get('digits'),
      dim,
    );
    digits = { type: digitType, values: explDigits };
  } else if (digitType == 'JCanonical' || digitType == 'JSymmetric') {
    const jValue = validateInputAsJValue(formData.get('param'), 'param', dim);
    digits = { type: digitType, jValue: jValue };
  } else if (digitType == 'Shifted') {
    const shift = validateInputAsPositiveInteger(
      formData.get('param'),
      'param',
    );
    digits = { type: digitType, shift: shift };
  } else {
    digits = { type: digitType };
  }

  const newSystem: NewSystem = {
    name,
    dimension: dim,
    base,
    digits,
  };

  return await dbErrorWrapper(
    () => insertSystem(newSystem, user.id),
    'Error uploading system',
  );
};

export const queueJob = async (formData: FormData) => {
  const user = await getLoggedInUser();

  const systemId = validateInputAsInteger(
    formData.get('system-id'),
    'system id',
  );
  const system = await gsId(systemId, user.id);
  if (!system) {
    throw new Error('This system does not exist');
  }
  const norm = validateInputAsNorm(formData.get('norm'), 'norm');
  const jobType = validateInputAsJobType(formData.get('job-type'), 'job type');

  const job: NewJob = {
    userId: user.id,
    systemId: systemId,
    status: 'Pending',
    jobType: jobType,
    norm: norm,
  };

  if (jobType == 'Path') {
    const point = validateInputAsVector(
      formData.get('grid-point'),
      system.dimension,
      'start point',
    );
    job.walkFrom = point;
  }

  return await dbErrorWrapper(() => insertJob(job), 'Error inserting job');
};
export const getJobs = async (params: JobsFilter) => {
  return await dbErrorWrapper(() => gj(params), 'Error getting jobs');
};

export const getSystemJobs = async (systemId: number) => {
  const filter = { systemId };
  return await getJobs(filter);
};

export const favourite = async (systemId: number) => {
  const user = await getLoggedInUser();

  return await dbErrorWrapper(
    () => fs(systemId, user.id),
    `Error favouriting system ${systemId}`,
  );
};

export const unFavourite = async (systemId: number) => {
  const user = await getLoggedInUser();

  return await dbErrorWrapper(
    () => ufs(systemId, user.id),
    `Error unfavouriting system ${systemId}`,
  );
};

async function dbErrorWrapper<T extends () => Promise<any>>(
  fn: T,
  errorStr: string,
): Promise<Awaited<ReturnType<T>>> {
  try {
    return await fn();
  } catch (e) {
    console.log(errorStr + ': ' + e);
    throw new Error('Database error');
  }
}

// Minio
export const getDownloadUrl = gdu;
