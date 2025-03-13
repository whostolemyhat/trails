import './style.css';

function setLoading(isLoading: boolean) {
  const submitButton = document.querySelector('#submit')!;
  const loadingEl = document.querySelector('#loading')! as HTMLElement;

  if (isLoading) {
    // disable button
    submitButton.setAttribute('disabled', 'disabled');
    // show loading state
    loadingEl.classList.add('isLoading');
  } else {
    submitButton.removeAttribute('disabled');
    loadingEl.classList.remove('isLoading');
  }
}

function setError(hasError: boolean) {
  const errorMsg = document.querySelector('#error')! as HTMLElement;

  if (hasError) {
    // show error msg
    errorMsg.classList.add('hasError');
  } else {
    errorMsg.classList.remove('hasError');
  }
}

const defaultOptions = {
  seed: String(Date.now()),
  minLeafSize: 3,
  canvasSize: 40,
  density: 2,
};
let cache = defaultOptions;
async function handleForm(e: SubmitEvent) {
  e.preventDefault();
  setError(false);
  setLoading(true);

  const data = new FormData(e.currentTarget as HTMLFormElement);

  const parsed = { ...defaultOptions, ...Object.fromEntries(data) };
  parsed.minLeafSize = Number(parsed.minLeafSize);
  parsed.canvasSize = Number(parsed.canvasSize);
  parsed.density = Number(parsed.density);

  if (JSON.stringify(parsed) !== JSON.stringify(cache)) {
    cache = parsed;

    try {
      const response = await fetch(`${import.meta.env.VITE_BASE_URL}/api/generate`, {
        method: 'post',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(parsed),
      });

      const result = await response.text();
      document.querySelector('#image')!.innerHTML = result;
    } catch (err) {
      console.error(err);
      setError(true);
    } finally {
      setLoading(false);
    }
  }

  setLoading(false);
}

function handleDownload() {
  const image = document.querySelector('#image')!.innerHTML;
  const svgBlob = new Blob([image], { type: 'image/svg+xml;charset=utf-8' });
  var svgUrl = URL.createObjectURL(svgBlob);
  const link = document.createElement('a');
  link.href = svgUrl;
  link.download = `${cache.seed}-${cache.density}-${cache.canvasSize}-${cache.minLeafSize}.svg`;
  link.click();
}

document.addEventListener('DOMContentLoaded', function ready() {
  const form = document.querySelector('#options');
  (form as HTMLFormElement)?.addEventListener('submit', handleForm);
  const downloadLink = document.querySelector('#download');
  downloadLink?.addEventListener('click', handleDownload);
});
