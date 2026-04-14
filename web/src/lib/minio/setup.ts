import { minioClient } from './adapter';

async function main() {
  const bucketName = process.env.MINIO_BUCKET;
  if (!bucketName || bucketName.length == 0) {
    console.log('MINIO_BUCKET env variable not set!');
    process.exit(1);
  }

  console.log('Checking for bucket ' + bucketName);
  const exists = await minioClient.bucketExists(bucketName);
  if (exists) {
    console.log('Bucket ' + bucketName + ' exists.');
  } else {
    await minioClient.makeBucket(bucketName);
    console.log('Bucket ' + bucketName + ' created.');
  }
}

main().catch((err) => {
  console.error('Bucket creation failed', err);
  process.exit(1);
});
