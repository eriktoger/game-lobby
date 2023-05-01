import { useState } from "react";
import styled from "styled-components";

export const wsUri = process.env.NEXT_PUBLIC_WS_URI;
export const baseUrl = process.env.NEXT_PUBLIC_API_URL;

async function createAccount({
  username,
  phone,
}: {
  username: string;
  phone: string;
}) {
  try {
    const url = `${baseUrl}/users/create`;
    let result = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, phone }),
    });
    return result.json();
  } catch (e) {
    return Promise.reject(e);
  }
}

async function signIn({ phone }: { phone: string }) {
  try {
    const url = `${baseUrl}/users/phone/${phone}`;
    let result = await fetch(url);
    return result.json();
  } catch (e) {
    return Promise.reject(e);
  }
}

const StyledForm = styled.form`
  display: flex;
  flex-direction: column;
  width: 320px;
  align-items: center;
  justify-content: center;
  .row-container {
    display: flex;
    label {
      width: 80px;
    }
  }
  .first-row {
    padding-bottom: 10px;
  }
  button {
    margin: 10px;
  }
`;

const StyledContainer = styled.div`
  height: 200px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  background-color: #484848;
  color: #bdbdbd;
  padding: 10px;
  border-radius: 10px;
  button {
    padding: 3px;
    color: black;
    background-color: #c8c8c8;
  }
`;

export default function Login({ setAuth }: any) {
  const [isShowSigIn, setShowSignIn] = useState(false);
  const showSignIn = () => {
    setShowSignIn((prev) => !prev);
  };

  const FormCreateUsername = ({ setAuth }: any) => {
    const onCreateUsername = async (e: any) => {
      e.preventDefault();
      let username = e.target.username.value;
      let phone = e.target.phone.value;
      if (username === "" || phone === "") {
        return;
      }
      let res = await createAccount({ username, phone });
      if (res === null) {
        alert("Failed to create account");
        return;
      }
      setAuth(res);
    };
    return (
      <>
        <StyledForm action="" onSubmit={onCreateUsername}>
          <div>
            <div className="first-row row-container">
              <label>Username: </label>
              <input
                required
                type="text"
                name="username"
                placeholder="John Doe"
              />
            </div>
            <div className="row-container">
              <label>Phone: </label>
              <input required type="text" name="phone" placeholder="+1111..." />
            </div>
          </div>
          <button type="submit">Submit</button>
        </StyledForm>
        <div>
          <p>
            Already have a username?{" "}
            <button onClick={showSignIn}>Sign In</button>
          </p>
        </div>
      </>
    );
  };

  const FormSignIn = ({ setAuth }: any) => {
    const onSignIn = async (e: any) => {
      e.preventDefault();
      let phone = e.target.phone.value;
      if (phone === "") {
        return;
      }
      let res = await signIn({ phone });
      if (res === null) {
        alert("Failed to create account");
        return;
      }
      if (!res.id) {
        alert(`Phone number not found ${phone}`);
        return;
      }
      setAuth(res);
    };
    return (
      <>
        <StyledForm action="" onSubmit={onSignIn}>
          <div className="row-container">
            <label>Phone: </label>
            <input required type="text" name="phone" placeholder="+1111..." />
          </div>
          <div>
            <button type="submit">Submit</button>
          </div>
        </StyledForm>
        <div>
          <p>
            Do not have a username? <button onClick={showSignIn}>Create</button>
          </p>
        </div>
      </>
    );
  };

  return (
    <StyledContainer>
      <h3>{isShowSigIn ? "Log in with your phone." : "Create your account"}</h3>
      {isShowSigIn ? (
        <FormSignIn setAuth={setAuth} />
      ) : (
        <FormCreateUsername setAuth={setAuth} />
      )}
    </StyledContainer>
  );
}
