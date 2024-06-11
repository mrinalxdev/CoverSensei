import React, { useState } from "react";
import type { TextContent, TextItem } from "pdfjs-dist/types/src/display/api";
import { MdCloudUpload } from "react-icons/md";

type Props = {
  setResumeText: React.Dispatch<React.SetStateAction<string>>;
  setIsLoading: React.Dispatch<React.SetStateAction<boolean>>;
};

const ResumeUploader: React.FC<Props> = ({ setResumeText, setIsLoading }) => {
  const [error, setError] = useState("");
  const [isDragOver, setIsDragOver] = useState(false);

  const mergeTextContent = (textContent: TextContent) => {
    return textContent.items
      .map((item) => {
        const { str, hasEOL } = item as TextItem;
        return str + (hasEOL ? "\n" : "");
      })
      .join("");
  };

  const readResume = async(pdfFile: File | undefined) => {
    const pdfjs = await import('pdfjs-dist');
    pdfjs.GlobalWorkerOptions.workerSrc = `//unpkg.com/pdfjs-dist@${pdfjs.version}/build/pdf.worker.min.mjs`

    if (!pdfFile) return ;

    const reader = new FileReader();

    reader.onload = async (event) => {
        const arrayBuffer = event.target?.result;
        if (arrayBuffer && arrayBuffer instanceof ArrayBuffer){
            const loadingTask = pdfjs.getDocument(new Uint8Array(arrayBuffer));
            loadingTask.promise.then(
                (pdfDoc) => {
                    pdfDoc.getPage(1).then((page) => {
                        page.getTextContent().then((textContent) => {
                            const extractedText = mergeTextContent(textContent);
                            setResumeText(extractedText);
                        })
                    })
                },
                (reason) => {
                    console.log(`Error during PDF loading : ${reason}`)
                }
            )
        }
    }

    reader.readAsArrayBuffer(pdfFile);
  }

  const handleDrop = async(event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    setResumeText("");
    setError("");
    setIsLoading(true);
  }





  return <div>ResumeUploader</div>;
};

export default ResumeUploader;
