import React from "react";
import { IoCaretBackOutline } from "react-icons/io5";
import { ROUTES } from "../utils/routes";

const Profile = ({ setPage, resume, setResume, OpenAIKey, setOpenAIKey }) => {
  const handleSubmit = (e) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const updatedResume = formData.get("resume");
    const updatedOpenAIKey = formData.get("openAIKey");

    setResume(updatedResume);
    setOpenAIKey(updatedOpenAIKey);
  };

  return (
    <div className="flex flex-col mx-5">
      <div className="flex flex-row justify-between my-3 items-center">
        <h2 className="text-2xl font-bold">Profile</h2>
        <button
          className="border mr-[1px] p-2 border-solid border-gray-600 rounded-[100%]"
          onClick={() => setPage(ROUTES.GENERATOR)}
        >
          <IoCaretBackOutline className="text-2xl" />
        </button>
      </div>

      <form className="flex-col" onSubmit={handleSubmit}>
        <div className="mb-6">
          <label
            htmlFor="openAIKey"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Your OpenAI key
          </label>
          <input
            type="text"
            name="openAIKey"
            id="openAIKey"
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg"
            placeholder="sk- ....1234"
            defaultValue={OpenAIKey}
            required
          />
        </div>
        <div className="mb-6">
          <label
            htmlFor="resume"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Your Resume
          </label>
          <textarea
            name="resume"
            id="resume"
            rows={8}
            className="block p-2.5 w-full text-gray-900 bg-gray-50 rounded-lg"
            placeholder="Place your resume here ..."
            defaultValue={resume}
          ></textarea>
        </div>
        <div className="mb-6 text-center">
          <button
            type="submit"
            className="border-2 border-solid border-blue-500 text-blue-500 text-lg rounded-md px-5 py-2"
          >
            Save
          </button>
        </div>
      </form>
    </div>
  );
};

export default Profile;
