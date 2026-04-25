import { createAsync } from '@solidjs/router';
import { getCurrentUser } from '~/api/server';
import Navbar from '~/components/shared/Navbar';

export default function Home() {
  const user = createAsync(() => getCurrentUser());

  return (
    <>
      <Navbar user={user() ?? null} />
      <main class="max-w-dvw">"TODO: Home page description"</main>;
    </>
  );
}
