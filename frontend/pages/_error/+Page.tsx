import { usePageContext } from "vike-react/usePageContext";

/**
 * Error fallback page (`/_error`).
 *
 * Rendered by Vike for any unhandled route or server error. Displays a
 * "Page Not Found" message for 404s and a generic "Internal Error" message
 * for all other error types.
 */
export default function Page() {
  const { is404 } = usePageContext();
  if (is404) {
    return (
      <>
        <h1>Page Not Found</h1>
        <p>This page could not be found.</p>
      </>
    );
  }
  return (
    <>
      <h1>Internal Error</h1>
      <p>Something went wrong.</p>
    </>
  );
}
