import { navigate } from "vike/client/router";

interface FooterNavProps {
  showBack?: boolean; // default true
}

export default function FooterNav({ showBack = true }: FooterNavProps) {
  const isLoggedIn = !!localStorage.getItem("userId");

  function handleBack() {
    navigate(isLoggedIn ? "/dashboard" : "/");
  }

  function handleLogout() {
    localStorage.removeItem("userId");
    localStorage.removeItem("secret");
    localStorage.removeItem("username");
    navigate("/");
  }

  return (
    <div className="fixed bottom-8 left-8 flex gap-3">
      {showBack && (
        <a
          onClick={handleBack}
          className="px-6 py-3 bg-gray-800 hover:bg-gray-700 rounded-lg text-base transition-colors cursor-pointer"
        >
          Back
        </a>
      )}
      {isLoggedIn && (
        <a
          onClick={handleLogout}
          className="px-6 py-3 bg-red-900 hover:bg-red-800 rounded-lg text-base transition-colors cursor-pointer"
        >
          Logout
        </a>
      )}
    </div>
  );
}
