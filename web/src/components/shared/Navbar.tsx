import { useUserContext } from '~/lib/UserContext';

export default function Navbar(props: {}) {
  const user = useUserContext();

  return <header class="sticky">Navbar</header>;
}
