import { useParams } from '@solidjs/router';

export default function SystemJobs() {
  return <main>Jobs for: {useParams().systemId}</main>;
}
