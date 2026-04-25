import SignUp from '~/components/auth/SignUp';
import { Title } from '@solidjs/meta';
import Navbar from '~/components/shared/Navbar';

export default function LoginPage() {
  return (
    <>
      <Title>Sign Up | Waves</Title>
      <Navbar user={null} />
      <main class="bg-background p-5 max-w-dvw">
        <SignUp />
      </main>
    </>
  );
}
