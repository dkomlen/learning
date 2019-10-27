import React from 'react'
import ClientCard from '../components/ClientCard'
import PostCard from '../components/PostCard'
import './index.css'

const IndexPage = () => (
  <div>
    <section className="portfolio">
      <div className="copy">
        <h1>Hi! My name is Malcom Voygt and I do copywrite</h1>
        <p>
          8888888888888888 ...by day. By night however, I write about Artificial
          Intelligence in pop culture
        </p>
        <a className="anchor" href="#contact">
          Drop me a line
        </a>
      </div>

      <div className="portfolio__featured">
        <ClientCard name="Skillfull" work="Class on copywriting techniques" />
        <ClientCard name="Skillfull" work="Class on copywriting techniques" />
        <ClientCard name="Skillfull" work="Class on copywriting techniques" />
        <ClientCard name="Skillfull" work="Class on copywriting techniques" />
        <ClientCard name="Skillfull" work="Class on copywriting techniques" />
        <ClientCard name="Skillfull" work="Class on copywriting techniques" />
      </div>
    </section>

    <section className="blog">
      <div className="copy">
        <h1>Not everything in life is work ... </h1>
        <p>
          Here's some blogging I do in AI to practice my words, just a silly
          thing I like to do
        </p>
      </div>
      <div className="blog__posts">
        <PostCard
          slug="not-gonna-work"
          title="Westworld"
          punchLine="Robots
      were never meant to live among us"
        />
        <PostCard
          slug="not-gonna-work"
          title="Westworld"
          punchLine="Robots
      were never meant to live among us"
        />
        <PostCard
          slug="not-gonna-work"
          title="Westworld"
          punchLine="Robots
      were never meant to live among us"
        />
        <PostCard
          slug="not-gonna-work"
          title="Westworld"
          punchLine="Robots
      were never meant to live among us"
        />
      </div>
    </section>
  </div>
)

export default IndexPage
