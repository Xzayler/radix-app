'use server';
import minioClient from './client';
const DOWNLOAD_URL_EXPIRY_SECONDS = 12 * 60 * 60;
const bucketName = process.env.MINIO_BUCKET!;

export async function getDownloadUrl(key: string): Promise<string> {
  try {
    await minioClient.statObject(bucketName, key);
  } catch (e) {
    console.log(e);
    throw new Error('Failed to find file');
  }

  try {
    return await minioClient.presignedGetObject(
      bucketName,
      key,
      DOWNLOAD_URL_EXPIRY_SECONDS,
      {
        'response-content-disposition': `attachment; filename="${key}"`,
        'response-content-type': 'application/octet-stream',
      },
    );
  } catch (e) {
    throw new Error('Failed to generate presigned url');
  }
}

// export async function uploadFile(object: string, content: string) {
//   await minioClient.putObject(bucketName, object, content);
// }
