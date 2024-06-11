"use client"
import React, { useEffect, useState } from 'react'
import ResumeWorth from './ResumeWorth';
import ResumeUploader from './ResumeUploader';
import {useCompletion} from 'ai/react';

const ResumeAnalyzer = () => {
  const [showWorth, setShowWorth] = useState(false);
  const [isLoadingResume, setIsLoadingResume] = useState(false);
  const [resumeText, setResumeText] = useState<string>("");
  const {completion, isLoading, complete, error} = useCompletion({
    api: '/api/resume',
  })
// useCompletion hook will help in getting the resume to the backend

useEffect(() => {
  const getResumeWorth = async(text: string) => {
    const messageToSend = `RESUME: ${text}\n\n-----\n\n`;
    await complete(messageToSend);
    setShowWorth(true);
    setIsLoadingResume(false);
  }

  if (resumeText !== ''){
    getResumeWorth(resumeText).then();
  }
}, [resumeText]);


    
  return (
    <div>
      {!showWorth ? (
        <div>
          <p>Upload your resume to know your worth</p>
          <ResumeUploader setIsLoading={setIsLoadingResume} setResumeText={setResumeText} />
        </div>
      ) : (
        <ResumeWorth />
      )}

      {error && <p>{error.message}</p>}
    </div>
  )
}

export default ResumeAnalyzer