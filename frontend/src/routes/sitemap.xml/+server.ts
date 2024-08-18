import { dev } from "$app/environment";

const url = dev ? "http://localhost:3000" : "https://disku.jniemela.dk";

function today_yyyy_mm_dd(): string {
    const d = new Date();
    const iso = d.toISOString();
    return iso.substring(0, 10);
}

function generate_xml(course_id: string): string {
    return `
    <url>
        <loc>${url}/course/${course_id}</loc>
        <priority>0.8</priority>
        <lastmod>${today_yyyy_mm_dd()}</lastmod>
    </url>
    `;
}

export async function GET() {
    // grab all course-ids from the get-course-ids endpoint which gives a list of json objects
    // [{course_id: "course1"}, {course_id: "course2"}]
    //
    const res = await fetch(`${url}/api/get-all-course-ids`);
    const json = await res.json();
    const today = today_yyyy_mm_dd();

    let course_ids = json.map((x: any) => x.course_id);
    // print how many course-ids we have
    console.log(`Found ${course_ids.length} course-ids for sitemap`);

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
        <lastmod>${today}</lastmod>
        <priority>1.0</priority>
      </url>
      <!-- course pages -->
      ${course_ids.map(generate_xml).join("\n")}

    </urlset>`.trim(),
        {
            headers: {
                "Content-Type": "application/xml",
            },
        }
    );
}
