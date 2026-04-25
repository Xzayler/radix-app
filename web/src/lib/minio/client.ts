'use server';
import { Client } from 'minio';

function validatedEnvVar(envVar: string): string {
  const value = process.env[envVar];
  if (!value || value.length == 0) {
    throw new Error(`${envVar} not set`);
  }
  return value;
}

function setupClient() {
  return new Client({
    endPoint: validatedEnvVar('MINIO_ENDPOINT'),
    port: parseInt(validatedEnvVar('MINIO_PORT')),
    useSSL: false,
    accessKey: validatedEnvVar('MINIO_USER'),
    secretKey: validatedEnvVar('MINIO_PASSWORD'),
  });
}

const minioClient = setupClient();
export default minioClient;
