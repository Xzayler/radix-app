import * as Minio from 'minio';

export const minioClient = new Minio.Client({
  endPoint: process.env.MINIO_ENDPOINT!,
  port: parseInt(process.env.MINIO_PORT!),
  useSSL: false,
  accessKey: process.env.MINIO_ROOT_USER!,
  secretKey: process.env.MINIO_ROOT_PASSWORD!,
});
const bucketName = process.env.MINIO_BUCKET!;

export async function uploadFile(object: string, content: string) {
  await minioClient.putObject(bucketName, object, content);
}
