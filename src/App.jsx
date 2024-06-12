import { useState } from "react";
import Generator from "./components/Generator";
import Profile from "./components/Profile";
import { ROUTES } from "./utils/routes";

function App() {
  const [page, setPage] = useState();
  const [resume, setResume] = useState("resume test");
  const [OpenAIKey, setOpenAIKey] = useState("openai Key test");

  switch (page) {
    case ROUTES.GENERATOR:
      return <Generator setPage={setPage} />;

    case ROUTES.PROFILE:
      return (
        <Profile
          setPage={setPage}
          resume={resume}
          setResume={setResume}
          OpenAIKey={OpenAIKey}
          setOpenAIKey={setOpenAIKey}
        />
      );

    default:
      return <Generator setPage={setPage} />;
  }
}

export default App;
