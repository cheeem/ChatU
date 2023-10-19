import { useState } from "react";

const INITIAL_STATE = {
  firstName: "",
  lastName: "",
  email: "",
  password: "",
  phoneNumber: "",
  instagram: "",
  snapchat: "",
  discord: "",
};

function SignUp() {
  const [formdata, setFormdata] = useState(INITIAL_STATE);

  function handleSubmit(e: React.FormEvent<EventTarget>) {
    e.preventDefault();
    console.log("Sign Up Successfully!");
    setFormdata(INITIAL_STATE);
  }

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    setFormdata({
      ...formdata,
      [e.target.id]: e.target.value,
    });
  }

  return (
    <>
      <h1>Create an account</h1>
      <form onSubmit={handleSubmit}>
        <fieldset>
          <legend>Basic Information</legend>
          <div>
            <label htmlFor="firstName">First Name: </label>
            <input
              type="text"
              name="firstName"
              id="firstName"
              required
              value={formdata.firstName}
              onChange={handleChange}
            />
          </div>
          <div>
            <label htmlFor="lastName">Last Name: </label>
            <input
              type="text"
              name="lastName"
              id="lastName"
              required
              value={formdata.lastName}
              onChange={handleChange}
            />
          </div>
          <div>
            <label htmlFor="email">Your School Email: </label>
            <input
              type="email"
              name="email"
              id="email"
              required
              placeholder="x500@umn.edu"
              value={formdata.email}
              onChange={handleChange}
            />
          </div>
          <div>
            <label htmlFor="password">Password: </label>
            <input
              type="password"
              name="password"
              id="password"
              required
              value={formdata.password}
              onChange={handleChange}
            />
          </div>
        </fieldset>

        <fieldset>
          <legend>Contact information(Optional)</legend>
          <div>
            <label htmlFor="phoneNumber">Phone Number: </label>
            <input
              type="tel"
              name="phoneNumber"
              id="phoneNumber"
              value={formdata.phoneNumber}
              onChange={handleChange}
            />
          </div>
          <div>
            <label htmlFor="instagram">Instagram: </label>
            <input
              type="text"
              name="instagram"
              id="instagram"
              value={formdata.instagram}
              onChange={handleChange}
            />
          </div>
          <div>
            <label htmlFor="snapchat">Snapchat: </label>
            <input
              type="text"
              name="snapchat"
              id="snapchat"
              value={formdata.snapchat}
              onChange={handleChange}
            />
          </div>
          <div>
            <label htmlFor="discord">Discord: </label>
            <input
              type="text"
              name="discord"
              id="discord"
              value={formdata.discord}
              onChange={handleChange}
            />
          </div>
        </fieldset>

        <button type="submit">Sign Up</button>
      </form>
    </>
  );
}

export default SignUp;
