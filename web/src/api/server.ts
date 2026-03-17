'use server';
import {
  login as li,
  getCurrentUser as gcu,
  logout as lo,
  register as rg,
  guestLogin as gl,
} from '~/lib/auth';
import {
  getSystemById as gsId,
  insertSystem,
  getSystems as gs,
  getJobs as gj,
  insertJob,
} from '~/lib/db/operations';
import { parseInputFile } from '~/lib/utils/fileParser';
import { NewJob, System } from '~/types';

// Auth
// TODO: Move formData handling here.
export const login = li;
export const getCurrentUser = gcu;
export const logout = lo;
export const register = rg;
export const guestLogin = gl;

// Db Entity operations
export const getSystemById = gsId;
export const getSystems = gs;
export const uploadSystem = async (formData: FormData): Promise<System> => {
  // try {
  const file = formData.get('input-file');
  if (!file || typeof file !== 'object') {
    throw new Error('No file provided');
  }
  if (file.type !== 'application/json') {
    throw new Error('File has to be JSON');
  }
  if (!file.text || typeof file.text !== 'function') {
    throw new Error('File has to have content');
  }
  const inputData = await parseInputFile(file);
  return await insertSystem(inputData);
};
export const queueJob = async (formData: FormData) => {
  const user = await getCurrentUser();
  if (!user) {
    throw new Error('No user logged in');
  }

  const systemIdEntry = formData.get('system-id');
  if (!systemIdEntry || typeof systemIdEntry !== 'string') {
    throw new Error('No system id provided');
  }
  const systemId: number = parseInt(systemIdEntry);
  if (isNaN(systemId)) {
    throw new Error('System id has to be an integer number');
  }

  const system = await gsId(systemId);
  if (!system) {
    throw new Error('This system does not exist');
  }

  const norm = formData.get('norm');
  if (!norm || typeof norm !== 'string') {
    throw new Error('No norm provided');
  }
  if (norm != 'Infinite' && norm != 'L1' && norm != 'L2') {
    throw new Error('Invalid norm value');
  }

  const jobType = formData.get('job-type');
  if (!jobType || typeof jobType !== 'string') {
    throw new Error('No job type provided');
  }

  if (
    jobType != 'Walk' &&
    jobType != 'Decision' &&
    jobType != 'Classification'
  ) {
    throw new Error('Invalid norm value');
  }

  const job: NewJob = {
    userId: user.id,
    systemId: systemId,
    status: 'Pending',
    jobType: jobType,
    norm: norm,
  };

  if (jobType == 'Walk') {
    const vectorString = formData.get('grid-point');
    if (!vectorString || typeof vectorString !== 'string') {
      throw new Error("A grid point must be provided for 'Walk' job type");
    }
    const values = vectorString.split(' ').map((s) => parseInt(s));
    if (values.length == system.dimension && values.every((v) => !isNaN(v))) {
      job.walkFrom = values;
    } else {
      throw new Error('Invalid grid point values');
    }
  }

  try {
    console.log('Attempting to insert job: ', job);
    return await insertJob(job);
  } catch (e) {
    console.log(e);
    throw new Error('Database error');
  }
};
export const getJobs = gj;
