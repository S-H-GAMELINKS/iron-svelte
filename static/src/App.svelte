<script>
    import { Router, Link, Route } from "svelte-routing";
    import index from "./routes/index.svelte";
    import about from "./routes/about.svelte";
    import contact from "./routes/contact.svelte";
    let url = "";
</script>

<Router url="{url}">
    <ul>
        <li>
            <Link to="/">Index</Link>
        </li>
        <li>
            <Link to="about">About</Link>
        </li>
        <li>
            <Link to="contact">Contact</Link>
        </li>
    </ul>    
    <div>
        <Route path="/" component="{index}" />
        <Route path="/about" component="{about}" />
        <Route path="/contact" component="{contact}" />
        <Route path="/"><index /></Route>
    </div>
</Router>