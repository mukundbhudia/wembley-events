#[cfg(test)]
pub fn serpapi_test_output_json_1() -> String {
    r##"
{
  "search_metadata": {
    "id": "629b5785af5af14447fa62f6",
    "status": "Success",
    "json_endpoint": "https://serpapi.com/searches/68c91323104d0721/629b5785af5af14447fa62f6.json",
    "created_at": "2022-06-04 13:00:53 UTC",
    "processed_at": "2022-06-04 13:00:53 UTC",
    "google_events_url": "https://www.google.com/search?q=Events+in+wembley&ibp=htl;events&uule=w+CAIQICIeV2VtYmxleSxFbmdsYW5kLFVuaXRlZCBLaW5nZG9t&hl=en",
    "raw_html_file": "https://serpapi.com/searches/68c91323104d0721/629b5785af5af14447fa62f6.html",
    "total_time_taken": 0.97
  },
  "search_parameters": {
    "q": "Events in wembley",
    "engine": "google_events",
    "location_requested": "Wembley, England, United Kingdom",
    "location_used": "Wembley,England,United Kingdom"
  },
  "search_information": {
    "events_results_state": "Results for exact spelling"
  },
  "events_results": [
    {
      "title": "Harry Styles",
      "date": {
        "start_date": "Jun 19",
        "when": "Sun, Jun 19"
      },
      "address": [
        "Club Wembley, Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://www.clubwembley.com/events/2022/harry-styles",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=oUNmLrW4QBiqSGiPv6KlME4_QHo1Y7-53fwLbWtrnaLHh5hJfjTFs-QrupOEp6-2cw5XU_KXKAza4NK0c1Mp8nwDOe5ZBAwOd56yqsEbqvT4_S3BUO8",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x4876122c53bcf57f:0x547e18f740779055?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x4876122c53bcf57f%3A0x547e18f740779055&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Multi-platinum recording artist Harry Styles announces the rescheduled dates for his world tour, in addition to new shows added across the globe. Styles will kick off his colossal 32-city outing...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1360986/harry-styles-love-on-tour/london/2022-06-19/en",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40205531-harry-styles-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/harry-styles-love-on-tour/wembley-stadium/2206022",
          "link_type": "tickets"
        },
        {
          "source": "The website for the English football association, the Emirates FA Cup and the England football team",
          "link": "https://www.clubwembley.com/events/2022/harry-styles",
          "link_type": "more info"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Harry%20Styles%20Pitch%20Standing%20-%2019th%20June",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Club Wembley",
        "rating": 4.5,
        "reviews": 255,
        "link": "https://www.google.com/search?hl=en&q=Club+Wembley&ludocid=6088331196469055573&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcShRkHc7cITjGfbMMkgj1tU-phjKHLT1ARe5VCNQJ9pww&s=10"
    },
    {
      "title": "David Guetta @ Wembley Stadium",
      "date": {
        "start_date": "Jun 12",
        "when": "Sun, 2 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://www.bandsintown.com/e/1024978668-david-guetta-at-wembley-stadium",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Capital's Summertime Ball",
      "ticket_info": [
        {
          "source": "Bandsintown.com",
          "link": "https://www.bandsintown.com/t/1024978668?came_from=209",
          "link_type": "tickets"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS7KQe3RBKFt-xf82bsCiVaYSNOF9Quv_FsktsUO5kzFw&s=10"
    },
    {
      "title": "Soulful Sundays",
      "date": {
        "start_date": "Jun 5",
        "when": "Tomorrow, 1 – 10 PM"
      },
      "address": [
        "BOXPARK Wembley, Wembley Park, Wembley",
        "Wembley, United Kingdom"
      ],
      "link": "https://wembleypark.com/whats-on/boxpark-wembley-soulful-sundays-5-june-2022/",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=FEe6dGnu8ufXnj6pFLtkyXymXM7QCYl4HZmD4Lpx-aiTzcvyrHGsleI4H5bbVKyoHQz4Imu_3wqqn_cSAn7ZGPONAAM5TQNBYObFmWO_6b4d05wvSW0",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x4876111ad33561d5:0xbdbb95733f14421d?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x4876111ad33561d5%3A0xbdbb95733f14421d&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Join BOXPARK Wembley and their resident DJ's for all things soulful",
      "ticket_info": [
        {
          "source": "Wembleypark.com",
          "link": "https://wembleypark.com/whats-on/boxpark-wembley-soulful-sundays-5-june-2022/",
          "link_type": "tickets"
        },
        {
          "source": "Boxpark",
          "link": "https://www.boxpark.co.uk/wembley/events/soulful-sundays-2/",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "BOXPARK Wembley",
        "rating": 4.4,
        "reviews": 3738,
        "link": "https://www.google.com/search?hl=en&q=BOXPARK+Wembley&ludocid=13671685416025342493&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQbmEOud03BmpPLdAJhrx6LFvzgc4Mj56E9knPNDsnmJQ&s=10"
    },
    {
      "title": "Ed Sheeran @ Wembley Stadium",
      "date": {
        "start_date": "Jun 29",
        "when": "Wed, 4:30 – 8:30 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%2029th%20June%202022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "span Ed Sheeran has announced the first leg of his ‘+ - = ÷ x Tour’ (pronounced ‘The Mathematics Tour’) , taking place in stadiums throughout 2022. Kicking off in April next year, the tour will...",
      "ticket_info": [
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40023151-ed-sheeran-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://edsheeran.seetickets.com/event/ed-sheeran-x-tour/wembley-stadium/2106169",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fed-sheeran-x-tour-london-06-29-2022%2Fevent%2F37005B3185CD1E37",
          "link_type": "tickets"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%2029th%20June%202022",
          "link_type": "more info"
        },
        {
          "source": "Belle Coaches",
          "link": "https://www.bellecoaches.co.uk/Tour/Ed-Sheeran-Travel-Only-Wembley",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSjK6Ygd7IF6T2ZkP3r1KzxyqBYfZ2YqBa1b2ZNmaDyCQ&s=10"
    },
    {
      "title": "Dylan @ Wembley Stadium",
      "date": {
        "start_date": "Jun 25",
        "when": "Sat, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%2025th%20June%202022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "span Ed Sheeran has announced the first leg of his ‘+ - = ÷ x Tour’ (pronounced ‘The Mathematics Tour’) , taking place in stadiums throughout 2022. Kicking off in April next year, the tour will...",
      "ticket_info": [
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40036221-ed-sheeran-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://edsheeran.seetickets.com/event/ed-sheeran-x-tour/wembley-stadium/2106206",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fed-sheeran-x-tour-london-06-25-2022%2Fevent%2F37005B35F23C4F24",
          "link_type": "tickets"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%2025th%20June%202022",
          "link_type": "more info"
        },
        {
          "source": "Eastons Holidays",
          "link": "https://www.eastonsholidays.co.uk/itineraries/6856-ed-sheeran-x-at-wembley-stadium",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSjK6Ygd7IF6T2ZkP3r1KzxyqBYfZ2YqBa1b2ZNmaDyCQ&s=10"
    },
    {
      "title": "Coldplay @ Wembley Stadium",
      "date": {
        "start_date": "Aug 16",
        "when": "Tue, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Coldplay%20Seated%2016th%20August%202022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Global super pop rock band Coldplay will be hitting the road on tour in 2022 and will be performing live on stage in their Music of the Spheres World Tour ! This particular live concert is...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1349863/coldplay-music-of-the-spheres-world-tour/london/2022-08-16/en",
          "link_type": "tickets"
        },
        {
          "source": "Ticketmaster.ie",
          "link": "https://www.ticketmaster.ie/wembley-stadium-hospitality-coldplay-london-16-08-2022/event/37005C41E63F31A3",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40062333-coldplay-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/coldplay-wembley-stadium-return-coach-travel/wembley-stadium/2129344",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fcoldplay-music-of-the-spheres-world-london-08-16-2022%2Fevent%2F37005B4DD1124221",
          "link_type": "tickets"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQA4bu9FAvVq7jNOix0UqQ-i0BRphX28gzb21K6iEQs3w&s=10"
    },
    {
      "title": "Ed Sheeran + - = ÷ x Tour",
      "date": {
        "start_date": "Jun 24",
        "when": "Fri, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/en/Ed-Sheeran-Hospitality-24th-June-2022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Ed Sheeran Seated Ticket & Hotel Experience This Experience Includes: • Reserved Seated Ticket (Block 104 or 141) Tickets seated together • One night hotel stay including complimentary breakfast •...",
      "ticket_info": [
        {
          "source": "Seetickets.com",
          "link": "https://edsheeran.seetickets.com/event/ed-sheeran-wembley-stadium-coach-travel-only/wembley-stadium/2113583",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fed-sheeran-x-tour-london-06-24-2022%2Fevent%2F37005B35EAB84F09",
          "link_type": "tickets"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/en/Ed-Sheeran-Seated-24th-June-2022",
          "link_type": "more info"
        },
        {
          "source": "Facebook",
          "link": "https://www.facebook.com/events/wembley-stadium-connected-by-ee/ed-sheeran-london/877103186301875/",
          "link_type": "more info"
        },
        {
          "source": "Club Sports and Events",
          "link": "https://www.clubsportsandevents.com/shop/all-events/music/concerts/ed-sheeran/ed-sheeran-friday-24th-june/",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSltTwhjki9s5yl8YmcyBxm5C74ZU55E6Fptazj9dOCYw&s=10"
    },
    {
      "title": "Coldplay: Music of the Spheres World Tour",
      "date": {
        "start_date": "Aug 20",
        "when": "Sat, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Coldplay%20Seated%2020th%20August%202022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Global super pop rock band Coldplay will be hitting the road on tour in 2022 and will be performing live on stage in their Music of the Spheres World Tour ! This particular live concert is...",
      "ticket_info": [
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40074133-coldplay-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/coldplay-music-of-the-spheres-world-tour/wembley-stadium/2121306",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fcoldplay-music-of-the-spheres-world-london-08-20-2022%2Fevent%2F37005B4EEF0C3CCB",
          "link_type": "tickets"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Coldplay%20Hospitality%2020th%20August%202022",
          "link_type": "more info"
        },
        {
          "source": "P1 Travel",
          "link": "https://www.p1travel.com/event/coldplay-20-aug/",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTiIiVMSSZLotUNu_d5yZrrDZv1pqHb0YKaHAWjCgu1UQ&s=10"
    },
    {
      "title": "Jubin Nautiyal - RESCHEDULED",
      "date": {
        "start_date": "Jun 26",
        "when": "Sun, 7 – 9 PM"
      },
      "address": [
        "OVO Arena, Arena Square, Engineers Way",
        "London, United Kingdom"
      ],
      "link": "https://wembleypark.com/whats-on/ovo-arena-wembley-jubin-nautiyal/",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=r8O_2LvdTQE6ezwNVH5c7B0vLyP1kjJEXdcg_6RTv_oF3H6CpnGV0TvfHCsnItErgUiLjAFbtN320oYjgvnzaZlQbem650ViyWmRIdFKgd2tf12Sv0M",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x487611971e81c5b7:0x614f1ff5c12a035?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x487611971e81c5b7%3A0x614f1ff5c12a035&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Jubin Nautiyal at OVO Arena Wembley in Wembley Park",
      "ticket_info": [
        {
          "source": "Wembleypark.com",
          "link": "https://wembleypark.com/whats-on/ovo-arena-wembley-jubin-nautiyal/",
          "link_type": "tickets"
        },
        {
          "source": "Axs.com",
          "link": "https://www.axs.com/uk/events/411160/jubin-nautiyal-rescheduled-tickets",
          "link_type": "tickets"
        },
        {
          "source": "Spotify - Web Player",
          "link": "https://open.spotify.com/concert/32uRr7BJ68vupEJiBTH7Ia",
          "link_type": "more info"
        },
        {
          "source": "Ticketmaster",
          "link": "https://www.ticketmaster.co.uk/jubin-nautiyal-ovo-arena-wembley-london-tickets/venueartist/451730/5275316",
          "link_type": "more info"
        },
        {
          "source": "Bandsintown",
          "link": "https://www.bandsintown.com/e/1023831745-jubin-nautiyal-at-ovo-arena-wembley",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "OVO Arena",
        "rating": 3.6,
        "reviews": 34,
        "link": "https://www.google.com/search?hl=en&q=OVO+Arena&ludocid=438241142825459765&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRo2p-iRfQz0MPHb3fqrxz2yTaXB7j3mZuE3suOZMDnew&s=10"
    },
    {
      "title": "50 Cent",
      "date": {
        "start_date": "Jun 10",
        "when": "Fri, 7:30 – 11:00 PM"
      },
      "address": [
        "The SSE Arena, Wembley, Arena Square, Engineers Way",
        "London, United Kingdom"
      ],
      "link": "https://www.facebook.com/events/ovo-arena-wembley/50-cent/1000089817594536/",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=oWSG54aadmWx5zmE3cA3_lqoYqXPGmDV7bVNc7Isd6dnqawz7-JnToYMjkcVZuP3qZ7rawGFA89FSZbIH3J_klODr4Jqduczsl6eey5x9IXAhIgpTEY",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0x3e4769509e11aea?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0x3e4769509e11aea&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "SJM Concerts 50 CENT FRI, 10 JUN 2022 at 07:30PM BST Doors Open: 06:00PM OnSale: Fri, 28 Jan 2022 at 09:30AM GMT Announcement: Wed, 26 Jan 2022 at 10:00AM GMT Hip Hop heavyweight 50 Cent, has...",
      "ticket_info": [
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/50-cent/ovo-arena-wembley/2202777",
          "link_type": "tickets"
        },
        {
          "source": "Facebook",
          "link": "https://www.facebook.com/events/ovo-arena-wembley/50-cent/1000089817594536/",
          "link_type": "more info"
        },
        {
          "source": "Ticketmaster",
          "link": "https://www.ticketmaster.co.uk/50-cent-ovo-arena-wembley-london/venueartist/451730/713910",
          "link_type": "more info"
        },
        {
          "source": "GIGSANDTOURS",
          "link": "https://www.gigsandtours.com/tour/50-cent",
          "link_type": "more info"
        },
        {
          "source": "AXS.com",
          "link": "https://www.axs.com/events/424041/50-cent-tickets",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "The SSE Arena, Wembley",
        "rating": 4.4,
        "reviews": 11530,
        "link": "https://www.google.com/search?hl=en&q=The+SSE+Arena,+Wembley&ludocid=280479459291765482&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSa3Ww5oQCAk9QhBKt8iMMsUytFVnGgWx8AgmPKb_or0w&s=10"
    }
  ]
}
    "##.to_string()
}

#[cfg(test)]
pub fn serpapi_test_output_json_2() -> String {
    r##"
    "##
    .to_string()
}

#[cfg(test)]
pub fn serpapi_test_output_json_3_some_fields_missing() -> String {
    r##"
{
  "search_metadata": {
    "id": "62b6c2b7cca014af0ea8e649",
    "status": "Success",
    "json_endpoint": "https://serpapi.com/searches/68c91323104d0721/62b6c2b7cca014af0ea8e649.json",
    "created_at": "2022-06-25 08:09:27 UTC",
    "processed_at": "2022-06-25 08:09:27 UTC",
    "google_events_url": "https://www.google.com/search?q=Events+in+wembley&ibp=htl;events&uule=w+CAIQICIeV2VtYmxleSxFbmdsYW5kLFVuaXRlZCBLaW5nZG9t&hl=en",
    "raw_html_file": "https://serpapi.com/searches/68c91323104d0721/62b6c2b7cca014af0ea8e649.html",
    "total_time_taken": 1.44
  },
  "search_parameters": {
    "q": "Events in wembley",
    "engine": "google_events",
    "location_requested": "Wembley, England, United Kingdom",
    "location_used": "Wembley,England,United Kingdom"
  },
  "search_information": {
    "events_results_state": "Results for exact spelling"
  },
  "events_results": [
    {
      "title": "Dylan @ Wembley Stadium",
      "date": {
        "start_date": "Jun 25",
        "when": "Sat, Jun 25, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/en/Ed-Sheeran-Seated-25th-June-2022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "span Ed Sheeran has announced the first leg of his ‘+ - = ÷ x Tour’ (pronounced ‘The Mathematics Tour’) , taking place in stadiums throughout 2022. Kicking off in April next year, the tour will...",
      "ticket_info": [
        {
          "source": "Seetickets.com",
          "link": "https://edsheeran.seetickets.com/rd/event/ed-sheeran-x-tour/wembley-stadium/2106206",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fed-sheeran-x-tour-london-06-25-2022%2Fevent%2F37005B35F23C4F24",
          "link_type": "tickets"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/en/Ed-Sheeran-Hospitality-26th-June-2022",
          "link_type": "more info"
        },
        {
          "source": "P1 Travel",
          "link": "https://www.p1travel.com/event/ed-sheeran/",
          "link_type": "more info"
        },
        {
          "source": "Eastons Holidays",
          "link": "https://www.eastonsholidays.co.uk/itineraries/6856-ed-sheeran-x-at-wembley-stadium",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.6,
        "reviews": 42956,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcThVFUTrxYeiBvbBb0KQd1OPMWg8fogf7rQN0wqoZqWbQ&s"
    },
    {
      "title": "Ed Sheeran",
      "date": {
        "start_date": "Jun 25",
        "when": "Today, 11 AM – 3 PM"
      },
      "address": [
        "The SSE Arena, Wembley, Arena Square, Engineers Way",
        "London, United Kingdom"
      ],
      "link": "https://ik-us.facebook.com/events/the-sse-arena-wembley/twenty-one-pilots-take%C3%B8ver-tour-london/860664711475431/",
      "description": "TWENTY ONE PILOTS: TAKEØVER TOUR - London 25 June 2022 Sign up for artist pre-sale at twentyonepilots.com/tour Sale Dates and Times: Public Onsale : Wed, 23 Jun 2021 at 09:00 AM O2 Presale :...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1337825/twenty-one-pilots-take%C3%B8ver-tour/london/2022-06-25/en",
          "link_type": "tickets"
        },
        {
          "source": "Seatgeek.com",
          "link": "https://seatgeek.com/twenty-one-pilots-tickets/london-uk-the-sse-arena-wembley-2022-06-25-7-pm/concert/5689837?utm_source=google&utm_medium=event-interface",
          "link_type": "tickets"
        },
        {
          "source": "Axs.com",
          "link": "https://www.axs.com/uk/events/405524/twenty-one-pilots-tickets",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Ftwenty-one-pilots-london-06-25-2022%2Fevent%2F37005ACBDB2557B0",
          "link_type": "tickets"
        },
        {
          "source": "Facebook",
          "link": "https://en-gb.facebook.com/events/ovo-arena-wembley/twenty-one-pilots-take%C3%B8ver-tour-london/860664711475431/",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "The SSE Arena, Wembley",
        "rating": 4.4,
        "reviews": 11651,
        "link": "https://www.google.com/search?hl=en&q=The+SSE+Arena,+Wembley&ludocid=280479459291765482&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQEoY0ZVtB6aLtGoNZVPVbSDNk5ib_mVjm_fzNUKXA&s"
    },
    {
      "title": "Ed Sheeran + - = ÷ x Tour",
      "date": {
        "start_date": "Jul 1",
        "when": "Fri, 9 AM – 1 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%201st%20July%202022",
      "description": "span Ed Sheeran has announced the first leg of his ‘+ - = ÷ x Tour’ (pronounced ‘The Mathematics Tour’) , taking place in stadiums throughout 2022. Kicking off in April next year, the tour will...",
      "ticket_info": [
        {
          "source": "Seetickets.com",
          "link": "https://edsheeran.seetickets.com/rd/event/ed-sheeran-x-tour/wembley-stadium/2106204",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fed-sheeran-x-tour-london-07-01-2022%2Fevent%2F37005B3185E31F04",
          "link_type": "tickets"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%201st%20July%202022",
          "link_type": "more info"
        },
        {
          "source": "Champions Travel - Ticket and Hospitality packages for the Premier League, La Liga and other major sporting events",
          "link": "https://www.champions-travel.com/tickets/?ticketid=3185",
          "link_type": "more info"
        },
        {
          "source": "P1 Travel",
          "link": "https://www.p1travel.com/event/ed-sheeran-1-juli/",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.6,
        "reviews": 42956,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ_rjTb2nYFaFl3MDGV8PJcxNxw4Fc9Q9xagLucmVo&s"
    },
    {
      "title": "Dylan @ Wembley Stadium",
      "date": {
        "start_date": "Jun 29",
        "when": "Wed, 4:30 – 8:30 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://www.wyomingnews.com/laramieboomerang/local-events?_evDiscoveryPath=/event%2F25604769t-ed-sheeran-official-ticket-hotel-package",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "span Ed Sheeran has announced the first leg of his ‘+ - = ÷ x Tour’ (pronounced ‘The Mathematics Tour’) , taking place in stadiums throughout 2022. Kicking off in April next year, the tour will...",
      "ticket_info": [
        {
          "source": "Seetickets.com",
          "link": "https://edsheeran.seetickets.com/rd/event/ed-sheeran-x-tour/wembley-stadium/2106169",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fed-sheeran-x-tour-london-06-29-2022%2Fevent%2F37005B3185CD1E37",
          "link_type": "tickets"
        },
        {
          "source": "Wyoming Tribune Eagle",
          "link": "https://www.wyomingnews.com/laramieboomerang/local-events?_evDiscoveryPath=/event%2F25604769t-ed-sheeran-official-ticket-hotel-package",
          "link_type": "more info"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Ed%20Sheeran%20Standing%2029th%20June%202022",
          "link_type": "more info"
        },
        {
          "source": "Belle Coaches",
          "link": "https://www.bellecoaches.co.uk/Tour/Ed-Sheeran-Travel-Only-Wembley",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.6,
        "reviews": 42956,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ0iCAszDAo46xrlfYzCeu0vVIf5Naulw2nsYuaWotgJZ_cwA0TFn2QhQZEBw&s=10"
    },
    {
      "title": "Neha Kakkar",
      "date": {
        "start_date": "Aug 28",
        "when": "Sun, 6:30 – 10:30 PM"
      },
      "address": [
        "The SSE Arena, Wembley, Arena Square, Engineers Way",
        "London, United Kingdom"
      ],
      "link": "https://wembleypark.com/whats-on/neha-kakkar-28-aug/",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=oWSG54aadmWx5zmE3cA3_lqoYqXPGmDV7bVNc7Isd6dnqawz7-JnToYMjkcVZuP3qZ7rawGFA89FSZbIH3J_klODr4Jqduczsl6eey5x9IXAhIgpTEY",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0x3e4769509e11aea?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0x3e4769509e11aea&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Please be advised that Wembley Arena seating is divided into bays: - Blocks A and B are not raked and other customers may stand during the performance. - Blocks C and D are tiered behind these. ...",
      "ticket_info": [
        {
          "source": "Wembleypark.com",
          "link": "https://wembleypark.com/whats-on/neha-kakkar-28-aug/",
          "link_type": "tickets"
        },
        {
          "source": "Axs.com",
          "link": "https://www.axs.com/uk/events/391315/neha-kakkar-rescheduled-tickets",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fneha-kakkar-london-08-28-2022%2Fevent%2F37005B4FAD4175D6",
          "link_type": "tickets"
        },
        {
          "source": "Facebook",
          "link": "https://m.facebook.com/events/428503702288585/",
          "link_type": "more info"
        },
        {
          "source": "Ticketmaster",
          "link": "https://www.ticketmaster.co.uk/search?q=neha",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "The SSE Arena, Wembley",
        "rating": 4.4,
        "reviews": 11651,
        "link": "https://www.google.com/search?hl=en&q=The+SSE+Arena,+Wembley&ludocid=280479459291765482&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTTUfFeAxwIEtdidilqEQeqLZC-Z3lGycLK1VfcgYE0Dg&s=10"
    },
    {
      "title": "Westlife at Wembley",
      "date": {
        "start_date": "Aug 6",
        "when": "Sat, Aug 6 – Sun, Aug 7"
      },
      "address": [
        "Best Western Palm Hotel, 64-76 Hendon Way",
        "London, United Kingdom"
      ],
      "link": "https://www.facebook.com/events/best-western-palm-hotel/westlife-at-wembley/2926639450957579/",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=9scHgGaqvDyca1Lup6cgzH2Oe7WF4dxtBhV_15O5ThZKThJNQbLTCAgeH14NGlQ-ESJ-w_H7PqEgs_bzxO3eMNrmG_VAZAw29lY95xqzs3yfqapWtw4",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x4876108e94cac0df:0x786294e3646518e9?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x4876108e94cac0df%3A0x786294e3646518e9&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Westlife first broke into the charts with their debut single Swear It Again. Released in April 1999, the song peaked at No.1 on the Official UK Singles Chart where it remained for two weeks. The...",
      "ticket_info": [
        {
          "source": "Facebook",
          "link": "https://www.facebook.com/events/best-western-palm-hotel/westlife-at-wembley/2926639450957579/",
          "link_type": "more info"
        },
        {
          "source": "Eat Out In Wembley",
          "link": "https://www.eatoutinwembley.co.uk/event/westlife-wembley-stadium-2022-august-06/",
          "link_type": "more info"
        },
        {
          "source": "AllEvents.in",
          "link": "https://allevents.in/london/swear%20it%20again",
          "link_type": "more info"
        },
        {
          "source": "Countdown Plus",
          "link": "https://www.countdownplusevents.com/events/westlife-5j0a0gbst2f",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Best Western Palm Hotel",
        "rating": 4,
        "reviews": 776,
        "link": "https://www.google.com/search?hl=en&q=Best+Western+Palm+Hotel&ludocid=8674659536631830761&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRl3-enNFu5v6cX1x7H2leD3YR15rKnHn4nSW8FZ_7RqQ&s=10"
    },
    {
      "title": "Coldplay: Music of the Spheres World Tour",
      "date": {
        "start_date": "Aug 20",
        "when": "Sat, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Coldplay%20Seated%2020th%20August%202022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Global super pop rock band Coldplay will be hitting the road on tour in 2022 and will be performing live on stage in their Music of the Spheres World Tour ! This particular live concert is...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1351175/coldplay-music-of-the-spheres-world-tour/london/2022-08-20/en",
          "link_type": "tickets"
        },
        {
          "source": "Ticketmaster.ie",
          "link": "https://www.ticketmaster.ie/wembley-stadium-hospitality-coldplay-london-20-08-2022/event/37005C41AEEF0D11",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40074133-coldplay-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/coldplay-music-of-the-spheres-world-tour/wembley-stadium/2121306",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fcoldplay-music-of-the-spheres-world-london-08-20-2022%2Fevent%2F37005B4EEF0C3CCB",
          "link_type": "tickets"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.6,
        "reviews": 42956,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQA4bu9FAvVq7jNOix0UqQ-i0BRphX28gzb21K6iEQs3w&s=10"
    },
    {
      "title": "Coldplay @ Wembley Stadium",
      "date": {
        "start_date": "Aug 16",
        "when": "Tue, 5 – 9 PM"
      },
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://ticketingcontent.thefa.com/Coldplay%20Seated%2016th%20August%202022",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Global super pop rock band Coldplay will be hitting the road on tour in 2022 and will be performing live on stage in their Music of the Spheres World Tour ! This particular live concert is...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1349863/coldplay-music-of-the-spheres-world-tour/london/2022-08-16/en",
          "link_type": "tickets"
        },
        {
          "source": "Ticketmaster.ie",
          "link": "https://www.ticketmaster.ie/wembley-stadium-hospitality-coldplay-london-16-08-2022/event/37005C41E63F31A3",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40062333-coldplay-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/coldplay-wembley-stadium-return-coach-travel/wembley-stadium/2129344",
          "link_type": "tickets"
        },
        {
          "source": "Tm7559.net",
          "link": "https://ticketmaster-uk.tm7559.net/c/253520/431519/7559?u=https%3A%2F%2Fwww.ticketmaster.co.uk%2Fcoldplay-music-of-the-spheres-world-london-08-16-2022%2Fevent%2F37005B4DD1124221",
          "link_type": "tickets"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.6,
        "reviews": 42956,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQA4bu9FAvVq7jNOix0UqQ-i0BRphX28gzb21K6iEQs3w&s=10"
    },
    {
      "title": "Celebrating Africa",
      "date": {
        "start_date": "Aug 27",
        "when": "Sat, 7 – 11 PM"
      },
      "address": [
        "The SSE Arena, Wembley, Arena Square, Engineers Way",
        "London, United Kingdom"
      ],
      "link": "https://www.seetickets.com/event/celebrating-africa/ovo-arena-wembley/2350132",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=oWSG54aadmWx5zmE3cA3_lqoYqXPGmDV7bVNc7Isd6dnqawz7-JnToYMjkcVZuP3qZ7rawGFA89FSZbIH3J_klODr4Jqduczsl6eey5x9IXAhIgpTEY",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0x3e4769509e11aea?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0x3e4769509e11aea&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Buy Celebrating Africa tickets and sign-up for latest tour alerts. Find Celebrating Africa tour dates, reviews, times and event details.",
      "ticket_info": [
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/celebrating-africa/ovo-arena-wembley/2350132",
          "link_type": "tickets"
        },
        {
          "source": "AXS.com",
          "link": "https://www.axs.com/events/435064/celebrating-africa-tickets",
          "link_type": "more info"
        },
        {
          "source": "Ents24",
          "link": "https://www.ents24.com/uk/tour-dates/sarkodie",
          "link_type": "more info"
        },
        {
          "source": "Stereoboard",
          "link": "https://www.stereoboard.com/celebrating-africa-tickets",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "The SSE Arena, Wembley",
        "rating": 4.4,
        "reviews": 11651,
        "link": "https://www.google.com/search?hl=en&q=The+SSE+Arena,+Wembley&ludocid=280479459291765482&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS3Gav1dU4FXhVtZIzXjs0PfCWGUAsGMj-z_VUJh3H-5g&s"
    },
    {
      "title": "Coldplay at Wembley",
      "date": {
        "start_date": "Aug 12",
        "when": "Aug 12 – 19"
      },
      "address": [
        "HA9 0WS",
        "Wembley, UK"
      ],
      "link": "https://www.biggreencoach.co.uk/events/coldplay-coach-travel-wembley-stadium-tickets",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=aXKFjtL7QQE_GmjX6JDp7uZI2wracCIQdhZwoZVulnV2LtqwoYd4Vo5GsU1M95-_5_LD28bfbQ",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xbc13cdefc41d64fa?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xbc13cdefc41d64fa&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Big Green Coach is delighted to be the official ticket and carbon neutral coach travel partner to the Coldplay Music of the Spheres World Tour at Wembley Stadium. VENUE: Wembley Stadium Dates...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1349857/coldplay-music-of-the-spheres-world-tour/london/2022-08-12/en",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40062327-coldplay-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/coldplay-wembley-stadium-return-coach-travel/wembley-stadium/2129333",
          "link_type": "tickets"
        },
        {
          "source": "Big Green Coach",
          "link": "https://www.biggreencoach.co.uk/events/coldplay-coach-travel-wembley-stadium-tickets",
          "link_type": "more info"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Coldplay%20Seated%2012th%20August%202022",
          "link_type": "more info"
        }
      ],
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRl9AkAFCyKwJ--DBWfpLnHSX9gWlxOb_HT6wvwL7G3A-MfgAPS3zwDkB08jA&s=10"
    }
  ]
}
    "##
    .to_string()
}

#[cfg(test)]
pub fn serpapi_test_output_json_4() -> String {
    r##"
      {
        "args": {}, 
        "headers": {
          "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8", 
          "Accept-Encoding": "gzip, deflate, br", 
          "Accept-Language": "en-GB,en;q=0.5", 
          "Dnt": "1", 
          "Host": "httpbin.org", 
          "Sec-Fetch-Dest": "document", 
          "Sec-Fetch-Mode": "navigate", 
          "Sec-Fetch-Site": "none", 
          "Sec-Fetch-User": "?1", 
          "Sec-Gpc": "1", 
          "Upgrade-Insecure-Requests": "1", 
          "User-Agent": "Mozilla/5.0 0 Firefox/101.0", 
          "X-Amzn-Trace-Id": "Root=1-62aee3e4-1e43e58103263fc73f548b84"
        }, 
        "origin": "0.0.0.0", 
        "url": "https://httpbin.org/get"
      }
    "##
    .to_string()
}

#[cfg(test)]
pub fn serpapi_test_output_json_5() -> String {
    r##"
      {
        "args": {}, 
        "headers": {
          "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8", 
          "Accept-Encoding": "gzip, deflate, br", 
          "Accept-Language": "en-GB,en;q=0.5", 
          "Dnt": "1", 
          "Host": "httpbin.org", 
          "Sec-Fetch-Dest": "document", 
    "##
    .to_string()
}

#[cfg(test)]
pub fn serpapi_test_output_json_6_missing_date() -> String {
    r##"
{
  "search_metadata": {
    "id": "629b5785af5af14447fa62f6",
    "status": "Success",
    "json_endpoint": "https://serpapi.com/searches/68c91323104d0721/629b5785af5af14447fa62f6.json",
    "created_at": "2022-06-04 13:00:53 UTC",
    "processed_at": "2022-06-04 13:00:53 UTC",
    "google_events_url": "https://www.google.com/search?q=Events+in+wembley&ibp=htl;events&uule=w+CAIQICIeV2VtYmxleSxFbmdsYW5kLFVuaXRlZCBLaW5nZG9t&hl=en",
    "raw_html_file": "https://serpapi.com/searches/68c91323104d0721/629b5785af5af14447fa62f6.html",
    "total_time_taken": 0.97
  },
  "search_parameters": {
    "q": "Events in wembley",
    "engine": "google_events",
    "location_requested": "Wembley, England, United Kingdom",
    "location_used": "Wembley,England,United Kingdom"
  },
  "search_information": {
    "events_results_state": "Results for exact spelling"
  },
  "events_results": [
    {
      "title": "Harry Styles",
      "date": {
        "start_date": "Jun 19",
        "when": "Sun, Jun 19"
      },
      "address": [
        "Club Wembley, Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://www.clubwembley.com/events/2022/harry-styles",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=oUNmLrW4QBiqSGiPv6KlME4_QHo1Y7-53fwLbWtrnaLHh5hJfjTFs-QrupOEp6-2cw5XU_KXKAza4NK0c1Mp8nwDOe5ZBAwOd56yqsEbqvT4_S3BUO8",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x4876122c53bcf57f:0x547e18f740779055?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x4876122c53bcf57f%3A0x547e18f740779055&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Multi-platinum recording artist Harry Styles announces the rescheduled dates for his world tour, in addition to new shows added across the globe. Styles will kick off his colossal 32-city outing...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1360986/harry-styles-love-on-tour/london/2022-06-19/en",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40205531-harry-styles-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/harry-styles-love-on-tour/wembley-stadium/2206022",
          "link_type": "tickets"
        },
        {
          "source": "The website for the English football association, the Emirates FA Cup and the England football team",
          "link": "https://www.clubwembley.com/events/2022/harry-styles",
          "link_type": "more info"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Harry%20Styles%20Pitch%20Standing%20-%2019th%20June",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Club Wembley",
        "rating": 4.5,
        "reviews": 255,
        "link": "https://www.google.com/search?hl=en&q=Club+Wembley&ludocid=6088331196469055573&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcShRkHc7cITjGfbMMkgj1tU-phjKHLT1ARe5VCNQJ9pww&s=10"
    },
    {
      "title": "David Guetta @ Wembley Stadium",
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://www.bandsintown.com/e/1024978668-david-guetta-at-wembley-stadium",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=BeLrxXVpFMekV5D1cGfHK5UUuzjyZVDvOtALeFPXfQdFdoVttXfFiEJ2gaV66vKPNKMypnbtGKBFixEeHJNjrrGKD00LhUt575JR1R1DKBVSNiAPMlE",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x48761181d57a876d:0xa64f9f185de8e097?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x48761181d57a876d%3A0xa64f9f185de8e097&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Capital's Summertime Ball",
      "ticket_info": [
        {
          "source": "Bandsintown.com",
          "link": "https://www.bandsintown.com/t/1024978668?came_from=209",
          "link_type": "tickets"
        }
      ],
      "venue": {
        "name": "Wembley Stadium",
        "rating": 4.5,
        "reviews": 42262,
        "link": "https://www.google.com/search?hl=en&q=Wembley+Stadium&ludocid=11983972060459753623&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS7KQe3RBKFt-xf82bsCiVaYSNOF9Quv_FsktsUO5kzFw&s=10"
    }
  ]
}
    "##.to_string()
}

#[cfg(test)]
pub fn serpapi_test_output_json_7_one_bad_json_event() -> String {
    r##"
{
  "search_metadata": {
    "id": "629b5785af5af14447fa62f6",
    "status": "Success",
    "json_endpoint": "https://serpapi.com/searches/68c91323104d0721/629b5785af5af14447fa62f6.json",
    "created_at": "2022-06-04 13:00:53 UTC",
    "processed_at": "2022-06-04 13:00:53 UTC",
    "google_events_url": "https://www.google.com/search?q=Events+in+wembley&ibp=htl;events&uule=w+CAIQICIeV2VtYmxleSxFbmdsYW5kLFVuaXRlZCBLaW5nZG9t&hl=en",
    "raw_html_file": "https://serpapi.com/searches/68c91323104d0721/629b5785af5af14447fa62f6.html",
    "total_time_taken": 0.97
  },
  "search_parameters": {
    "q": "Events in wembley",
    "engine": "google_events",
    "location_requested": "Wembley, England, United Kingdom",
    "location_used": "Wembley,England,United Kingdom"
  },
  "search_information": {
    "events_results_state": "Results for exact spelling"
  },
  "events_results": [
    {
      "title": "Harry Styles",
      "date": {
        "start_date": "Jun 19",
        "when": "Sun, Jun 19"
      },
      "address": [
        "Club Wembley, Wembley Stadium",
        "London, United Kingdom"
      ],
      "link": "https://www.clubwembley.com/events/2022/harry-styles",
      "event_location_map": {
        "image": "https://www.google.com/maps/vt/data=oUNmLrW4QBiqSGiPv6KlME4_QHo1Y7-53fwLbWtrnaLHh5hJfjTFs-QrupOEp6-2cw5XU_KXKAza4NK0c1Mp8nwDOe5ZBAwOd56yqsEbqvT4_S3BUO8",
        "link": "https://www.google.com/maps/place//data=!4m2!3m1!1s0x4876122c53bcf57f:0x547e18f740779055?sa=X&hl=en",
        "serpapi_link": "https://serpapi.com/search.json?data=%214m2%213m1%211s0x4876122c53bcf57f%3A0x547e18f740779055&engine=google_maps&google_domain=google.com&hl=en&q=Events+in+wembley&type=place"
      },
      "description": "Multi-platinum recording artist Harry Styles announces the rescheduled dates for his world tour, in addition to new shows added across the globe. Styles will kick off his colossal 32-city outing...",
      "ticket_info": [
        {
          "source": "Livenation.co.uk",
          "link": "https://www.livenation.co.uk/show/1360986/harry-styles-love-on-tour/london/2022-06-19/en",
          "link_type": "tickets"
        },
        {
          "source": "Songkick.com",
          "link": "https://www.songkick.com/concerts/40205531-harry-styles-at-wembley-stadium?utm_medium=organic&utm_source=microformat",
          "link_type": "tickets"
        },
        {
          "source": "Seetickets.com",
          "link": "https://www.seetickets.com/event/harry-styles-love-on-tour/wembley-stadium/2206022",
          "link_type": "tickets"
        },
        {
          "source": "The website for the English football association, the Emirates FA Cup and the England football team",
          "link": "https://www.clubwembley.com/events/2022/harry-styles",
          "link_type": "more info"
        },
        {
          "source": "Tickets - The FA",
          "link": "https://ticketingcontent.thefa.com/Harry%20Styles%20Pitch%20Standing%20-%2019th%20June",
          "link_type": "more info"
        }
      ],
      "venue": {
        "name": "Club Wembley",
        "rating": 4.5,
        "reviews": 255,
        "link": "https://www.google.com/search?hl=en&q=Club+Wembley&ludocid=6088331196469055573&ibp=gwp%3B0,7"
      },
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcShRkHc7cITjGfbMMkgj1tU-phjKHLT1ARe5VCNQJ9pww&s=10"
    },
    {
      "title": "David Guetta @ Wembley Stadium",
      "address": [
        "Wembley Stadium",
        "London, United Kingdom"
  ]
}
    "##.to_string()
}
