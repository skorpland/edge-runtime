import { createClient } from "npm:@powerbase/powerbase-js@2.42.0";
Deno.serve((_req) => new Response(`${typeof createClient}`));
