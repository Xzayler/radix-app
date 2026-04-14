import { useParams } from '@solidjs/router';

export default function SystemDetails() {
  return <main>System Details for {useParams().systemId}</main>;
}
