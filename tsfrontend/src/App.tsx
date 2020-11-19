import React, { useState, useEffect } from "react";
import { useForm } from "react-hook-form";
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link,
  Redirect,
  useParams,
} from "react-router-dom";

// <Route path="/pastes/:id">
//   <GetPaste />
// </Route>

export default function App() {
  return (
    <Router>
      <Switch>
        <Route exact path="/">
          <UploadPaste />
        </Route>
        <Route path="/:id" children={<GetPaste />} />
        <Route path="*">
          <Redirect to="/" />
        </Route>
      </Switch>
    </Router>
  );
}

type Inputs = {
  pastedata: string;
};

const styles = {
  invalidData: {
    color: "red",
  } as React.CSSProperties,
};

export function UploadPaste() {
  const { register, handleSubmit, errors } = useForm<Inputs>();
  const [redirectTo, setRedirectTo] = useState("");
  const onSubmit = async (data: Inputs) => {
    console.log(data);

    const response = await fetch("http://3.250.0.143:8080/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
      },
      body: "body=" + encodeURIComponent(data.pastedata),
    });

    const id = new TextDecoder("utf-8").decode(
      (await response.body.getReader().read()).value
    );
    console.log(id);
    setRedirectTo(id);
  };

  if (redirectTo != "") {
    const path: string = "/" + redirectTo;
    return <Redirect to={path} />;
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <textarea
        name="pastedata"
        placeholder="Enter your paste here..."
        rows={60}
        cols={120}
        ref={register({ required: true })}
      />
      <br />
      <input type="submit" />
      {errors.pastedata && (
        <span style={styles.invalidData}>Please enter some text to store!</span>
      )}
    </form>
  );
}

type urlParams = {
  id: string;
};

export function GetPaste() {
  const { id } = useParams<urlParams>();
  const [bodytext, setBodyText] = useState("");
  const [redirectHome, setRedirectHome] = useState(false);

  useEffect(() => {
    async function get_paste(id: string) {
      const response = await fetch("http://3.250.0.143:8080/" + id, {
        method: "GET",
      });

      if (response.status == 500) {
        setRedirectHome(true);
      }
      const bodytext = new TextDecoder("utf-8").decode(
        (await response.body.getReader().read()).value
      );
      setBodyText(bodytext);
    }
    get_paste(id);
  }, []);

  if (redirectHome) {
    return <Redirect to="/" />;
  }
  return <div>{bodytext}</div>;
}
