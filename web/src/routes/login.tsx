import Login from '~/components/auth/Login';
import { Title } from '@solidjs/meta';
import Navbar from '~/components/shared/Navbar';

export default function LoginPage() {
  return (
    <>
      <Title>Login | Waves</Title>
      <Navbar user={null} />
      <main class="bg-background p-5 max-w-dvw">
        <Login />
      </main>
    </>
  );
}
