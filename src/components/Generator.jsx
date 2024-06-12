import React from "react";
import { CiSettings } from "react-icons/ci";
import { ROUTES } from "../utils/routes";

const Generator = ({ setPage }) => {
  return (
    <div className="flex flex-col">
      <div className="flex flex-row justify-between mx-5 my-3 items-center">
        <button className="border-2 border-solid border-blue-500 text-blue-500 text-lg font-semibold p-2 rounded-md hover:bg-blue-500 hover:text-white duration-100 transition-all">
          Generate
        </button>
        <h2 className="text-2xl font-bold">
          CoverSensei 🥷 | Generate Cover Letter
        </h2>
        <button className="border mr-[1px] p-2 border-solid border-gray-600 rounded-full" onClick={() => setPage(ROUTES.PROFILE)}>
          <CiSettings className="text-2xl" />
        </button>
      </div>
      <div className="flex mx-5">
        <textarea
          rows={12}
          className="w-full"
          placeholder="Generated cover letter"
        />
      </div>
    </div>
  );
};

export default Generator;
