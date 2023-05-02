import { useState } from "react";
import styled from "styled-components";
import { createAccount, signIn } from "api";

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
      if (res == null) {
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
    const [error, setError] = useState("");
    const onSignIn = async (e: any) => {
      e.preventDefault();
      let phone = e.target.phone.value;
      if (phone === "") {
        return;
      }
      let res = await signIn({ phone });
      if (res === null) {
        setError("Failed to create account");
        return;
      }
      if (res.error === 404) {
        setError(res.message);
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
          {error && <div>Error:{error}</div>}
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
