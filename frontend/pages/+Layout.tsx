import "./Layout.css";
import { useEffect, useState } from "react";
import { usePageContext } from "vike-react/usePageContext";
import { navigate } from "vike/client/router";

const PUBLIC_ROUTES = ["/", "/login", "/create_account", "/leaderboard"];

export default function Layout({ children }: { children: React.ReactNode }) {
  const { urlPathname } = usePageContext();
  const [authChecked, setAuthChecked] = useState(false);

  useEffect(() => {
    if (PUBLIC_ROUTES.includes(urlPathname)) {
      setAuthChecked(true);
      return;
    }
    const userId = localStorage.getItem("userId");
    const secret = localStorage.getItem("secret");
    if (!userId || !secret) {
      navigate("/");
    } else {
      setAuthChecked(true);
    }
  }, [urlPathname]);

  if (!authChecked) return null;

  return <div className="">{children}</div>;
}
