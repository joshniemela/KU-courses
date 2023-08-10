// look in the data folder and get the names of all files in the jsons
// grab jsons from ../data/json if in devmode, otherwise from /data/json in prob
import { dev } from "$app/environment";

const jsons_dir = dev ? "../data/json" : "jsons";

const url = "https://disku.jniemela.dk";

import * as fs from 'fs';

function all_course_names(): string[] {
  return fs.readdirSync(jsons_dir).map((x) => x.replace(".json", ""))
}


function today_yyyy_mm_dd(): string {
  const d = new Date();
  const iso = d.toISOString();
  return iso.substring(0, 10);
}

function generate_xml(course_name: string): string {
  return `
    <url>
        <loc>${url}/course/${course_name}</loc>
        <priority>0.8</priority>
        <lastmod>${today_yyyy_mm_dd()}</lastmod>
    </url>
    `
}



export async function GET() {
  return new Response(
    `
    <?xml version="1.0" encoding="UTF-8" ?>
    <urlset
      xmlns="https://www.sitemaps.org/schemas/sitemap/0.9"
      xmlns:xhtml="https://www.w3.org/1999/xhtml"
      xmlns:mobile="https://www.google.com/schemas/sitemap-mobile/1.0"
      xmlns:news="https://www.google.com/schemas/sitemap-news/0.9"
      xmlns:image="https://www.google.com/schemas/sitemap-image/1.1"
      xmlns:video="https://www.google.com/schemas/sitemap-video/1.1"
    >

      <!-- root -->
       <url>
        <loc>${url}</loc>
        <lastmod>${today_yyyy_mm_dd()}</lastmod>
        <priority>1.0</priority>
      </url>
      <!-- course pages -->
      ${all_course_names().map(generate_xml).join("\n")}

    </urlset>`.trim(),
    {
      headers: {
        "Content-Type": "application/xml",
      },
    }
  )
}
