document.addEventListener("DOMContentLoaded", () => {
  const setVideoPosition = () => {
    const rect = canvas.getBoundingClientRect();

    video.style.top = rect.top + "px";
    video.style.left = rect.left + "px";
    video.style.width = rect.width + "px";
    video.style.height = rect.height + "px";

    videoBG.style.top = rect.top + "px";
    videoBG.style.left = rect.left + "px";
    videoBG.style.width = rect.width + "px";
    videoBG.style.height = rect.height + "px";
  };

  const button = document.querySelector("button");

  // console.log(button);
  // console.log(canvas);
  // console.log(video);

  // Get the canvas element and its position & size
  const canvas = document.querySelector("canvas");
  if (!canvas) {
    console.error("Canvas not found");
    return;
  }

  // Create the video element
  const videoContainer = document.createElement("div");
  videoContainer.style.display = "none";

  const video = document.createElement("video");
  video.setAttribute("src", "video/screaming.mp4");
  video.setAttribute("preload", "auto");
  video.style.position = "absolute";
  video.style.zIndex = "2"; // place it above the canvas

  // Create the div with id=video-bg
  const videoBG = document.createElement("div");
  videoBG.id = "video-bg";
  videoBG.style.position = "absolute";
  videoBG.style.zIndex = "1"; // place it below the video but above the canvas
  videoBG.style.backgroundColor = "black";

  setVideoPosition();

  videoContainer.appendChild(video);
  videoContainer.appendChild(videoBG);
  document.body.appendChild(videoContainer);

  // Optionally adjust canvas zIndex to ensure it's at the bottom
  canvas.style.zIndex = "0";

  button.addEventListener("click", () => {
    videoContainer.style.display = "block";
    videoBG.style.display = "block";
    video.play();
  });

  video.addEventListener("ended", () => {
    videoContainer.style.display = "none";
    videoBG.style.display = "none";
  });

  window.addEventListener("resize", () => {
    setVideoPosition();
  });
});
