import axios from "axios";

export default function Page() {
    async function makeUser() {
      let response = await axios.post(
          'https://create-user-91972588391.us-central1.run.app/',
        {
          "username": "testuser",
          "password": "password123"
        }
      );
      console.log(response);
    }

  return (
    <>
      <h1>Hello World</h1>
      <button onClick={makeUser}> Make a User Test </button>
    </>
  );
}
