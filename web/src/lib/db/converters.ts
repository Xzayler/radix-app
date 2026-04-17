import {
  AdjoinedDigits,
  CanonicalDigits,
  DenseDigits,
  Digits,
  ExplicitDigits,
  JCanonicalDigits,
  Job,
  JSymmetricDigits,
  NewJob,
  NewSystem,
  ShiftedDigits,
  SymmetricDigits,
  System,
} from '~/types';
import {
  JobDbEntity,
  JobDbInsert,
  SystemDbEntityWithDetails,
  SystemDbInsert,
} from './dbTypes';

function convertDigits(system: SystemDbEntityWithDetails): Digits {
  switch (system.digitType) {
    case 'Explicit':
      const e: ExplicitDigits = {
        type: 'Explicit',
        values: system.digits!,
      };
      return e;
    case 'Canonical':
      const c: CanonicalDigits = {
        type: 'Canonical',
      };
      return c;
    case 'JCanonical':
      const jc: JCanonicalDigits = {
        type: 'JCanonical',
        jValue: system.digitParam!,
      };
      return jc;
    case 'Adjoined':
      const a: AdjoinedDigits = {
        type: 'Adjoined',
      };
      return a;
    case 'Dense':
      const p = system.digitParam;
      const d: DenseDigits = {
        type: 'Dense',
        normType: p == 1 ? 'L1' : p == 2 ? 'L2' : 'Infinite',
      };
      return d;
    case 'Shifted':
      const sh: ShiftedDigits = {
        type: 'Shifted',
        shift: system.digitParam!,
      };
      return sh;
    case 'Symmetric':
      const sy: SymmetricDigits = {
        type: 'Symmetric',
      };
      return sy;
    case 'JSymmetric':
      const js: JSymmetricDigits = {
        type: 'JSymmetric',
        jValue: system.digitParam!,
      };
      return js;
  }
}

function chunkArray(array: number[], n: number) {
  return Array.from({ length: n }, (_, i) => array.slice(i * n, i * n + n));
}

export function systemFromDbEntity(
  dbEntity: SystemDbEntityWithDetails,
): System {
  const dim = dbEntity.dimension;
  const system: System = {
    id: dbEntity.id,
    dimension: dim,
    base: chunkArray(dbEntity.base, dim),
    digits: convertDigits(dbEntity),
    isGns: dbEntity.isGNS,
    signature: dbEntity.signature,
    lastJob: dbEntity.lastJob,
    isFavourited: dbEntity.isFavourited,
  };
  return system;
}

function getDigitParam(digits: Digits): number | null {
  switch (digits.type) {
    case 'JCanonical':
    case 'JSymmetric':
      return digits.jValue;
    case 'Shifted':
      return digits.shift;
    case 'Explicit':
    case 'Canonical':
    case 'Dense':
    case 'Adjoined':
    case 'Symmetric':
      return null;
  }
}

export function dbInsertFromSystem(
  system: NewSystem,
  digitIds?: number[],
): SystemDbInsert {
  const dbEntity: SystemDbInsert = {
    dimension: system.dimension,
    base: system.base.flat(),
    digitType: system.digits.type,
    // Digit Parameters
    digitIds: digitIds ?? null,
    digitParam: getDigitParam(system.digits),
  };
  return dbEntity;
}

export function jobFromDbEntity(
  dbEntity: JobDbEntity,
  gridPoint?: number[] | null,
): Job {
  const job: Job = {
    id: dbEntity.id,
    userId: dbEntity.userId,
    systemId: dbEntity.systemId,
    status: dbEntity.status,
    jobType: dbEntity.jobType,
    norm: dbEntity.norm,
    walkFrom: gridPoint ?? undefined,
    outputUri: dbEntity.outputUri ?? undefined,
    createdAt: dbEntity.createdAt,
    startedAt: dbEntity.startedAt ?? undefined,
    finishedAt: dbEntity.startedAt ?? undefined,
  };
  return job;
}

export function dbInsertFromJob(
  job: NewJob,
  gridPointId?: number,
): JobDbInsert {
  const dbEntity: JobDbInsert = {
    userId: job.userId,
    systemId: job.systemId,
    status: job.status,
    jobType: job.jobType,
    norm: job.norm,
    walkFrom: gridPointId,
  };
  return dbEntity;
}
