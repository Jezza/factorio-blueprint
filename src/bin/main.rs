fn main() {
	use factorio_blueprint::*;



//	let input = r#"0eNrFVNFOwzAM/Bc/d6jtxAYRfAlCVdq6w6JNKteZqKb+O0mKpolRGJMQL43s5M72XZMDlK3DnskIqANQZc0A6ukAA+2MbkNOxh5BAQl2kIDRXYhqrKhGXlW2K8losQxTAmRqfAOVTc8JoBESwpktBmNhXFci+wNHnlBPtJFTogR6O3isNaG851v582NYfImaGKt5L08CXNi2RYkvek8e6wENtYK8MMWeWJzPHBuYT6zKVlevYYTKuiBFdjpMTBsz1x0CWxY+O0Y0p+NRPTdFXDmSGAb0NCVnCuTfKXkmQHpz+xsFPigLv1fTseeGeJDiGkVmi0Bt0hB1vebYpoIHj7JOencdbz8WUe6iYdsVZDwPKGGH04LijPUPeufh1/vSmNnH/DKabJFmfZm/66v8Tf/V3s/uPv6Bu41uh0V7L7lQeRTcuxPfI3XyfCWw99c+Crddp9nd/WaTZttpegfSMq7i"#;
//	let input = r#"0eNqlk8FqwzAMht9FZ2ckzdZuvg123guMEhxHW8ViOzh2WQl+98kJZIVStrKLQbb+X59leYK2jzh4sgHkBKSdHUG+TTDSh1V93gunAUECBTQgwCqTI+UpHAwG0oV2piWrgvOQBJDt8AtklfYC0AYKhIvhHJwaG02LnhNWK4MdRVNgjzp49htcj1xocCOLnc0IbFjcCziBLLkEM1rOpRl1giovHx7RntehDuSGc8nrSGEJ0z6lJC5QNr/c6pKlvHtYaTryCwzI7cwWvOubFg/qSCxmxY9rw8cdreTv5MfQXHT6SD5E3lmxlowClT7kFo+Ybf6ua3ulP7PQDejVggrPry+c6GIY4s0I6ZZHqLnrAjZXz6srr1LfOCD1v+djIeG5nUddnv0MAUf041xrV5fV49N2W1a7lL4Bt+4cZA=="#;
//	let input = r#"0eNqVUtFuwjAM/Bc/l4kyAVveJvG8H5hQlaYeWGuTKnHQEMq/z2kkQIJp4yWq07vzXewTtH3E0ZNlUCcg42wA9XGCQDur+3zHxxFBATEOUIHVQ660MXGIvWbnIVVAtsNvUHXaVoCWiQmLzFQcGxuHFr0A7gpUMLogHGdzP9GZzZ+WFRzLh8iLK/aub1rc6wMJQ2Au8hi5ufF5IM9Rbs6dCmK2hFSULJrcKmROnY+dR7TXbqkDtRAseROJS5m2Sfg3gRaXQJ54PyCTmRk3tGT/iFaLm458MQNqdT/lRbWR3x2dnX+SDw+kR232eVABs8z/eW2vzVcmuhG9Llbh7X0jwEcHUCzcGcLi1yHU06vLTk3Lp652tYID+jDZWT/P65fX1Wper1P6AYqC9v4="#;
//	let input = "0eNqVkd9KxTAMxt8l151siDtazpvIYbRddMG1HW02HGPvbrqBCl6ovcuf75cvzQZ2nHFKFBj0BuRiyKCfN8j0GsxYcrxOCBqI0YOCYHyJTCIePDK5ykVvKRiOCXYFFHp8B93sNwUYmJjwBB7B2oXZW0zS8AtKwRSzqGMoHoRY1XcPClbQtUzpKaE7i60CMc0pjp3FwSwkYlF8UTsp9wcpl8ILpczdj/UWSjxL5tPW2VGhcUPZK2PB/F1nR+PeijBOmMxpFa5X6YszT/O/HciTHz2OoL/dTMGCKR/0y33dPD61bd1c9v0D2S+ivA==";
//	let input = "0eNp9kMFqw0AMRP9lzmuwCSTp/koIYe0qrcDWmrVsYsz+e7UOFF/ai0DiaWakDW0/05hYFH4Dd1Em+NuGib8k9GWm60jwYKUBDhKG0hVOg2jVxaFlCRoTsgPLJ73gm3x3IFFWprfc3qwPmYeWkgH/CjmMcbLdKMXf9GqH1aoZGK4p9o+WvsPCxhrw5F4p/RF74aSzTX4N30RVY1eby92nY/Bcsu/H+sNvHBaz2BNdTnVz/Tif6+aS8w95qWyQ";
//	let input = "0eNqVUc1OwzAMfhefM6nV0AY58CIIVWlrmMWaVI5TUaq+O05KGRLiwCGRbH/+fpIF2mvCkckL2AWoCz6CfVog0qt319yTeUSwQIIDGPBuyFUU170dyEdkQYbVAPke38HW67MB9EJCuDGVYm58GlpF2vovDgNjiLoWfFZVqsrArLdyqyvhcG1avLiJAmdAR9wlkkZn/ffWC3GU5pf3iViSdm7SBXG4g41cneT8VS6G0bGTrAGPZfylw+j65uJ8nxVFQ2k64YQ3RBkW2BB6LEn3UUQ9Oa56+8B9cevs4ciP6R/ej7Cu+a3Lv9gf32hgQo7lQc7Hqr5/OJ2q+ryun0Y5q/8=";
//	let input = "0eNqVUdtKxDAQ/Zd5TqFlZVfz4I+IhLQ71mHbpEwm1VLy7yYtrjcEfUmY2zlnzqzQDhEnJiegV6DOuwD6YYVAvbNDyckyIWggwREUODuWKIjtLhW5gCzIkBSQO+Mr6CY9KkAnJIQ70hYsxsWxzZ26+Q1DweRDHvOusGaoWsGS34ydVQn7wbT4bGfyXBo64i6SmFw7X6eeiIOYH9pnYok580G9dVQ3sINnJWX/ugTjZNlK4YD7Uh58T0Go+xPRV5NitoR79vmvWhxkZ/tOkHEddmLEmyuXQ3nxfAEtHFHtTpl3F8hN8R9LHiClcpRNm/50bwUzctgWOh3q5vbueKybU0pvRN+5hg==";
//	let input = "0eNqVUttOwzAM/Rc/d1KroQ3ywI8gFKWt2ay1SeU4gzH133FSbcAQEjw0lW/nHJ/kDO2QcGLyAuYM1AUfwTydIdLOuyHn5DQhGCDBESrwbsxRFNcdVuQjsiDDXAH5Ht/ANPNzBeiFhHBBKsHJ+jS22mma3zAqmELUseAzq0LVFZz0VGxVJRwG2+LeHSlwbuiIu0RitdZfp16Io9gf2o/EkjTzSV06VnewgKuSvH+dg3Fy7CRzwGMuD2FHUaj7E9F3k5JawjsO+l+1OMjCdkuguB47sRLslcujvAY+gBFOWF13ZXS93TvfZzGixsbbjlIsbWPosbh9KUXUL1uust/xMrhkLgaTn9I//FvDPOf7LmubL0+pgiNyLF5t13Vz/7DZ1M12nj8A7HDcAw==";
//	let input = "0eNqVUdtKxDAQ/Zd5TqFlZVfz4I+IhLQ71mHbpEwm1VLy7yYtrjcEfUmY2zlnzqzQDhEnJiegV6DOuwD6YYVAvbNDyckyIWggwREUODuWKIjtLhW5gCzIkBSQO+Mr6CY9KkAnJIQ70hYsxsWxzZ26+Q1DweRDHvOusGaoWsGS34ydVQn7wbT4bGfyXBo64i6SmFw7X6eeiIOYH9pnYok580G9dVQ3sINnJWX/ugTjZNlK4YD7Uh58T0Go+xPRV5NitoR79vmvWhxkZ/tOkHEddmLEmyuXQ3nxfAEtHFHtTpl3F8hN8R9LHiClcpRNm/50bwUzctgWOh3q5vbueKybU0pvRN+5hg==";
//	let input = "0eNqVUttugzAM/Rc/pxKoU7vlYT8yTRFQl1mFBDkOW4f49zmgtrto0vZAkG/nHJ9kgrpLODB5ATsBNcFHsE8TRGp91eWcnAcECyTYgwFf9TmKUjWnDfmILMgwGyB/wDew5fxsAL2QEK5IS3B2PvW1dtryNwwDQ4g6FnxmVajCwFlPxVZVwqFzNb5UIwXODQ1xk0ic1g7XqSNxFPdD+0gsSTM36qVjcwcruCrJ+xc56IeKK8kc8JjLXWgpCjV/IvpqUlJLuOWg/02Nnaxs3wkU12MjToK7cnmU18AnsMIJzXXXPhzQhaMLAyrEImV7q0bUL3uqut7xMrtmLg6SH9I/DNrCPOcLXfayn96KgRE5Lgr226K8f9jtinI/zx96MdBV";
//	let input = "0eNqVUttqwzAM/Rc9O5CQ0W5+2I+MYXJRM9HEDrLcrpT8++SEdTcG24uNbuccHfsK7ZhwZvIC9grUBR/BPl0h0uCbMefkMiNYIMEJDPhmylGUpjsW5COyIMNigHyPr2Cr5dkAeiEh3JDW4OJ8mlrttNVvGAbmEHUs+MyqUKWBi56KraqEw+hafGlOFDg3dMRdInFa629TB+Io7of2E7EkzXxQrx3FHWzgqiTvX+ZgmhtuJHPAYy6PYaAo1P2J6KtJSS3hgYPeRYujbGzfCRTXYydOgrtxeZRz4CNY4YTmtusUenTh4MKMCrFKqc3mo3v3iPyc/mFBDcuSn2xVbj/9BgMn5Lhy7Ouyun/Y7cpqvyxvIiTEvg==";
//	let input = "0eNplj9EKwjAMRf8lz1VWxKn9FZGyblEDW1OybjhG/912exF8yw33ntys4PoJg5CPYFaglv0I5r7CSC/f9GUXl4BggCIOoMA3Q1HCjgNLhKSAfIcfMDo9FKCPFAl3xiYW66fBoWTDf1pB4DEH2JdLGXKojmcFyz5kdu4ThXvr8N3MxFJsLUk7UbQDd2j5aTmgNDtDp1Jiq2p+PlMwo4yb43Kq9PVW15W+pPQFSttWFQ==";
//	let input = "0eNqtktFOwzAMRf/FzxnqhNigv4JQlGRmWKRx5DoVVdV/J2klQNoje4ut6+NrOwv4WDALJYV+AQqcRuhfFxjpmlxsOZ0zQg+kOICB5IYWCXvOLAqrAUoX/IL+uL4ZwKSkhDtjC2abyuBRquC22kDmsRZwap0q5NA9PBmY90dlVz8qHK3HDzcRS5MFklBI7cAXtPxuOaO4nVFbuMlRdD6ijXylUSlYLpqL2puRJhItNfPja1ccfHThs02mrC7egfPrqe1XpYRm97++7sFa29G20/Z/foKBCWXcNnp+7I7PL6dTdzyv6zfCesVx";
//	let input = "0eNqtkt1OwzAMhd/F1xnqhNigr4JQlGRmWKRx5DoVVdV3J2klQNolu4ut4y/HPwv4WDALJYV+AQqcRuhfFxjpmlxsOZ0zQg+kOICB5IYWCXvOLAqrAUoX/IL+uL4ZwKSkhDtjC2abyuBRquC22kDmsRZwaj9VyKF7eDIw74/Krn5UOFqPH24iliYLJKGQ2oEvaPndckZxO6Mz4CZH0fmINvKVRqVguWguam9amki01MyPr11x8NGFz9aZsrp4B86vpzZflRKa3f/6ugdrbUvbVtv/uQQDE8q4TfT82B2fX06n7nhe12/BG8Vw";
//	let input = "0eNptUEFqxDAM/MucvZBQ2G39jt5KMU6itoLEDrIcGkL+XjuBpUt7sdFoNDPShm7MNAsHhd3AfQwJ9m1D4s/gx4rpOhMsWGmCQfBTrVQ8h0vSOGM34DDQN2y7vxtQUFamU+UoVhfy1JEUwn/zBnNMZSSG6lZkGoO1vEW3pFGJo+voyy8cpRJ6lj6zOgq+G8kNnOoPq5LJQMgP7kPi5A6HB7i6zTQ8do7i3vqz9cKiuSD34Cfj8oq95Evqz9xAXf04kf11UYOFJB2M21PTPr9cr0172/cfrBSBxA==";
//	let input = "0eNptjsEKwjAQRP9lzhFahFbzKyKS6iIL7SYkW2kp+XeT9uLBy8IMM29nwzDOFCKLwm7gp5cEe9uQ+C1urJ6ugWDBShMMxE1VaXQsp6Q+IBuwvGiBbfPdgERZmQ7KLtaHzNNAsQT+9Q2CT6XipX4rmMZgLbdwk7rDByp6n2B/Fht8KKY90Z+b9nLtuqbtc/4CiaRGvQ==";
//	let input = "0eNptkMFqxDAMRP9lzl5IKOxu/R29lWKcRm0FiWxsJTQE//vaCZRCexFIjN6MtGOYFoqJRWF38HuQDPu6I/On+KnNdIsEC1aaYSB+bp0mz3LJGiKKActI37B9eTMgUVamk3I0m5NlHihVwX/7BjHkuhKkuVVMZ7DVWrk1jaYwuYG+/MohNUEmGZ0GdxBgP/yUyZw813iRRvcn/MpJlzr58T8VlxeUapPVn/ZAu+C41P56jMFKKR+K21PX35+v166/lfIA6olsFQ==";
//	let input = "0eNqNUdFqxCAQ/Jd99iDHwV0r/ZNSxMRNu9SorJvQEPLvVdMehfahL8rOzu7M6Aa9nzExBQG9AQ0xZNDPG2R6DdZXTNaEoIEEJ1AQ7FQrYUvhlCUm2BVQcPgB+ry/KMAgJITHllasJsxTj1wIf80rSDGXkRiqWlnTKVjLWfYWN8LRmx7f7EKRK2EgHmYSU3ruPjUSZzG/PC/EMhfkLnswTr23wzscAllszd7VYkqWrVQdeGrtLy0MtvdoHOV6gxaeUUHG4IxE07KAHq3PBWW0zowcp2/8ILfC1MAJ3f+dXmAvPorHIylAfeL2FfrHzylYkHNj3C7d+eHxeu3Ot33/BJV9pX4=";
//	let input = "0eNqNktFugzAMRf/Fz6kEdGu3/MpUoQBWZy0kKHHYEMq/j4BasRWpPDq+PvZ1PEKlA3aODIMcgWprPMiPETxdjdLpjYcOQQIxtiDAqDZFnq3Bw7fSGqIAMg3+gMyjeFp4VYyrkiJeBKBhYsKl8RwMpQlthW5ibrUU0Fk/lViT+kyYQyFgAJlN4MkBO6vLCj9VT9YlRU2uDsSl7dCU8wSSXUBxTzhUTenR+KRfUjZwF7h8sNOT46D0ahWz4lBpVX9BjGkH/zwUf90/Tp/fp2/IYb2kig3S8Qkp2wt62bXW21Y3AK+7ADdj6ZfnO5CrexPQo/Oz+HzM8rf30ynLzzH+AioO3cs=";
//	let input = "0eNqNk9FOwzAMRf/Fz5nUdrBBxJ8gVKWtNyzSpEqcjmnqv5O0bAxWtD46to99b5ITVDpg58gwyBNQbY0H+XoCT3ujdDrjY4cggRhbEGBUmyLP1uDqoLSGQQCZBj9B5oO427hXjFctxfAmAA0TE06Dx+BYmtBW6CJzbqSAzvrYYk2aEzGrQsARZBbBUQE7q8sK31VP1qWKmlwdiEvboSnHDSS7gOKScKia0qPxqX5K2cBd4PJGTk+Og9JXVowVq0qr+iMpOzPjIs1lxx05PwPb6UDND+oQV3MwifCs0p1kKWg75RSn3eAFhiHZ/MemYplN62+bZgjr31d025tfLG7IYT2lihnSwx1SthT0uEhU8b+mzSLAWVh6iuNjlVefQkCPzo/F23WWPz1vNlm+HYYvcaIVsg==";
//	let input = "0eNp9kE1qAzEMhe+itQszFJLWVyllkBN1YuIfYXvSmsF3r+yQ0m66MbwnvU+WdjBuI042FNA72FMMGfTbDtmuAV33SmUCDbaQBwUBfVec4prQezSOnjITXilBU2DDmb5Az+1dAYVii6U7b4i6hM0b6dTz/yQFHLOEY+g/EOCkoMorExilmwql3EvssBo8XZdbdFvnCfjHW1006JwEP9BlUiAifi4cXeVLDA+/9QKlsvxF54v0jgLokjaJDyeGxSM/rHvQU8649m0wn6H15ce59K/rKrgJeGx0fJ7ml9fDYZqPrX0DydeHrw==";

//	let input = "0eNp9UEFOxDAM/IvP2VW3EuwS8ROEorQ1YKlxosRZUVX5+zqFAyeOHs+MZ7zDtFZMmVjA7kBz5AL2bYdCn+zXjsmWECyQYAAD7EOfFpxpwXyaY5iIvcQMzQDxgt9gL+3dALKQEP64HcPmuIYJsxL+8zGQYlFp5H5d7QYDG9jTcH7SExpQclzdhF/+TkpXzq+J091yCEtHexXxvdfQZSH5fPhbeIUOpE0FlcV95BgccapKlVyxtZ7/6Gv/vMfAHXM5Yj3fruN4u7wM17G1B92qb48=";
//	let input = "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";
	let input = "0eNptUF1qwzAMvoueXUjzsHa+yhjBSbVW4J8gy6Eh+AA7yC62k0xOYRtsL4ZP/n70aYPRF5yZooDdgKYUM9iXDTJdo/NtJuuMYIEEAxiILjSUg/P+4F2YoRqgeME72GN9NYBRSAgfLjtYh1jCiKyE//QG5pRVkmJLU5vOwKqv+uo2wskPI97cQokbYSKeCsmgf5dv1RtxluHPzguxFJ38xO6Mg4ur3Che4ZGRxbX6XQNhduykRcHn+0cjlIwa5hNrJeGCtbXcr2F/Hc/Agpz3dZ7Op74/H5+7U1/rF2+LejQ=";
//	let input = "0eNqVk9FugzAMRf/FrwsT0NJN/MpUoRDc1RIkKDhVqyr/viRoHdu6tX1BsmNf35yYM7S9w9GSZqjPQMroCeq3M0z0rmUfc3waEWogxgEEaDnESFri/YBMKlNmaElLNha8ANIdHqEuvLipEWex1HxdofRbAaiZmHC2lIJTo93Qog0j/hUSMJop9Bod5x9T+QnqLBiDjiyq+WgtYjdb0zct7uWBQmuo31HPaP8gcSDLLmQu8+eKrIjulXERZbG4h3hAo1xolBeNld+mtNaz7ykqFfFjsVuyoS61KbLKEaewCL0+eviBr7zxlr8AZvlzlRjm3xFuriP8Um3CcUcX3zuyEzd3E0Gp9hHKhFGm+XxrqKtcgBnRytkGPIVW43h0D4v7++HONMNmplWuF3+PgEPYmGTlZZUXVVW9FuuN9x/bZCep";
//	let input = "0eNp9UEtOxDAMvYvXGdQCBRRxE4SitDFgqXGifEZUVe6Ok2HBiqWf/X4+Yd0rxkRcQJ9AW+AM+u2ETJ9s946VIyJooIIeFLD1fXK4kcN02YJfiW0JCZoCYoffoOf2rgC5UCG8qY3hMFz9ikkO/tNREEMWauDuLnKX6W5RcICexMJRwu22vFcgcUsKu1nxy15JyML4lTSyc0Mmd7QXK7a3nDrNR5uGm4ZX6EA8hFC5mI8UvCGOVU5LqthabzPa6z/PUnDFlEeO54dpXpblZX58au0H6cB0JQ==";
//	let input = "0eNptUNtqwzAM/Rc9u5DmYW39sB8ZIzip1gp8CbIcGoL/fXIK3WB7EUg6Fx1tMPqCM1MUsBvQlGIG+7FBplt0vs1knREskGAAA9GF1uXgvD94F2aoBihe8QH2WD8NYBQSwqfK3qxDLGFEVsB/fANzykpJsbmpTGdg1aq6eo1w8sOId7dQ4gaYiKdCMuju+mJ9EWcZ/ty8EEvRyY/tjji4uMqd4g2eHllci9+1JsyOnTQreG/rklGtfGINJFywtoz7L+yv1xlYkPN+zNv51Pfn46U79bV+Ay7teGI=";
//	let input = "0eNq9lF9OwzAMxu/i5xR15c9GxU0QqtLWWy1ap0rcQTXlANyCB07GSUhahCaBgA20p8qu/dnfT0l2ULYD9pZYIN8BVYYd5Lc7cLRh3cacjD1CDiTYgQLWXYxqrKhGm1SmK4m1GAteAXGNj5Av/J0CZCEhnNWmYCx46Eq0oeA7HQW9caHVcJwe5JJMwRg+6dllmBE2FGvaosRGbynUh6J3lSL8q6dOF7Nrsk6KH4w8GFMjJ1WDTmCWd6IjjTQGXa/ttFUONxAT/RimDCzF2pquIO6HUCp2QO+9+mQ0O8zo4l+NbsnKEDIfO8wVCW7RjtIQb6Jhh1Hs993ZTGkPzOvTyxFozg9Ck56EjObjuZStru6/YPN8BJuLg9ic6NToqvnrBQnPwnT78r1XR0E4jm5yc7VaZtlqcZ0uM+/fAOU2ngQ=";
//	let input = "0eNq9VduK2zAQ/Rc920vsXHZraKGQPpY+FPpSgpHtcXZYWTKyFGqCP6D/0S/rl3RkbxInsbNJCvuSMCPPmaMzB82WJcJCqVGaOFHqhUXbQ6Zi0c9e6M4wVbJLV7iWXLicqUtgEUMDBfOY5IWLMkgxA+2nqkhQcqM0azyGMoNfLAoa702AnFfGr0qBxkC/NmxWHgNp0CB0TNqgjqUtEvoyCvYQBWRoCx8EpEZj6pdKADUoVUXFSrrWBOjPPVbTX0g96HaSPsb2klsWuB8NWb8NZo6DdxTPmlXT9HI7KuGNVGY3MwlOmEypEnVq0bwSHSY2vTSnc1oPnUQT4pWh7mixaNGyNFqJOIFnvkGqpYJXxJjOMtxfIEddmfhs5hvUxlJmz6f7wk8ET19Yp0NluDPfxAVFyXVLMmIfqUhZU9obYJ2CLWhZEz9Lps+1KmKUhMKinIsKmuvFn5+L7bFw1DPDo5gdOBZcCF/wohwwaWeNoBnWfEdjr/kdknNZm2eU68uqf3LHtgJqJZQmeYy2cItfh1WY32XI4NiQ4fsacnGqzd/ff+7wZA950JWtwCMKrzWAfNOWx0/E4qJNTx6QwL2213QZG+xif92dcBcnO71hrjkKWg0j2+gKta0bYtBbSqv/ULm7P4nVbrGotzU9JngCxI19/r6kaEOcu/fz6TEMn4IPk8fwsN4mQxh+u5oPQF+/Lf3llx+U4UR1A/GueAS9+QdlYrXb";

	{
		let value = import_as_value(input).unwrap();
//		println!("{:#?}", value);
		println!("========================================================================================");
		println!("{:?}", value);
		println!("========================================================================================");
	}

	let obj = import(input).unwrap();
	println!("{:#?}", obj);
}

