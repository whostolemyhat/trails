import './style.css'

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

let cache = '';
async function handleForm(form: HTMLFormElement) {
  form.addEventListener('submit', async (e) => {
    e.preventDefault();
    setError(false);
    setLoading(true);

    const data = new FormData(e.currentTarget as HTMLFormElement);
    const defaultOptions = {
      seed: String(Date.now()),
      minLeafSize: 3,
      canvasSize: 40,
      density: 2
    };

    const parsed = {...defaultOptions, ...Object.fromEntries(data)};
    parsed.minLeafSize = Number(parsed.minLeafSize);
    parsed.canvasSize = Number(parsed.canvasSize);
    parsed.density = Number(parsed.density);


    if (JSON.stringify(parsed) !== cache) {
      cache = JSON.stringify(parsed);

      try {
        const response = await fetch(`${import.meta.env.VITE_BASE_URL}/api/generate`,{
          method: 'post',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(parsed)
        });

        const result = await response.text();
        document.querySelector('#image')!.innerHTML = result;
      } catch (err) {
        console.error(err);
        setError(true);
      }
    }

    setLoading(false);
  });
}


handleForm(document.querySelector('#options')!);