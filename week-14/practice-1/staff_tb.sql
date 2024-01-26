--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    no integer NOT NULL,
    age integer NOT NULL,
    mobile character varying NOT NULL,
    staff_sal real
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, no, age, mobile, staff_sal) FROM stdin;
100	Mustapha Ali	3	32	8063285831	175000
107	Alokwe Martin	7	48	7090082812	380000
97	Dankade Aminat	5	40	9023688832	550000
108	Josiah Joshua	1	30	8053189131	120000
102	Mankinde Mary	2	55	9023487830	450000
120	Adeleke Jane	4	38	7061045862	200000
122	Osahon Mark	6	44	8022289842	320000
104	Kuti Lawal	1	35	9145689842	750000
117	Suleman Ajayi	3	50	7030089981	800000
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

