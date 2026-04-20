'use server';
import {
  login as li,
  getCurrentUser as gcu,
  logout as lo,
  register as rg,
  guestLogin as gl,
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
  validateFileName,
  validateInputAsFile,
  validateInputAsJobType,
  validateInputAsNorm,
  validateInputAsNumber,
  validateInputAsVector,
} from '~/lib/validators';
import { getDownloadUrl as gdu } from '~/lib/minio/adapter';
import { NewJob, System } from '~/types';

// Auth
// TODO: Move formData handling here.
export const login = li;
export const getCurrentUser = gcu;
export const logout = lo;
export const register = rg;
export const guestLogin = gl;

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
export const uploadSystem = async (formData: FormData): Promise<System> => {
  const user = await getLoggedInUser();

  const file = validateInputAsFile(formData.get('input-file'));
  const name = validateFileName(formData.get('name'), 'name');
  const inputData = await parseInputFile(file);
  inputData.name = name;

  return await dbErrorWrapper(
    () => insertSystem(inputData, user.id),
    'Error uploading system',
  );
};
export const queueJob = async (formData: FormData) => {
  const user = await getLoggedInUser();

  const systemId = validateInputAsNumber(
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

  if (jobType == 'Walk') {
    const point = validateInputAsVector(
      formData.get('grid-point'),
      system.dimension,
      'grid point',
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
