// @refresh reload
import { Router } from '@solidjs/router';
import { FileRoutes } from '@solidjs/start/router';
import { Suspense } from 'solid-js';
import './app.css';
import { MetaProvider } from '@solidjs/meta';
import Loading from './components/shared/Loading';

export default function App() {
  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Suspense fallback={<Loading />}>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
